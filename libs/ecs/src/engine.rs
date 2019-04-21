use crate::{ComponentStore, System, SystemMut};
use frunk::{hlist, Hlist};

/// An `Engine` that wraps a trait object.
type BoxedEngine = Engine<Box<dyn SystemMut>>;

/// Wraps a `ComponentStore` and several systems.
#[derive(Debug)]
pub struct Engine<P: SystemMut> {
    /// The `ComponentStore` being wrapped.
    pub cs: ComponentStore,
    passes: P,
}

impl Engine<Hlist![]> {
    /// Creates an engine with no systems.
    pub fn new() -> Engine<Hlist![]> {
        Engine {
            cs: ComponentStore::new(),
            passes: hlist![],
        }
    }
}

impl<P: SystemMut> Engine<P> {
    /// Adds a `SystemMut` as a pass.
    pub fn add_mut_pass<T: SystemMut>(self, system: T) -> Engine<Hlist![Mut<T>, ...P]> {
        self.map_passes(|p| hlist![Mut(system), ...p])
    }

    /// Starts building a parallel pass.
    pub fn build_par_pass(self) -> EnginePassBuilder<P, Hlist![]> {
        EnginePassBuilder {
            engine: self,
            pass: hlist![],
        }
    }

    /// Maps the `passes` variable.
    fn map_passes<F: FnOnce(P) -> T, T: SystemMut>(self, func: F) -> Engine<T> {
        Engine {
            cs: self.cs,
            passes: func(self.passes),
        }
    }

    /// Runs the engine for one "turn," which encompassing running all systems once.
    pub fn run_once(&mut self) {
        self.passes.run(&mut self.cs)
    }
}

impl<P: 'static + SystemMut> Engine<P> {
    /// Converts the engine to use a trait object as its bound.
    pub fn boxed(self) -> BoxedEngine {
        self.map_passes(|ps| -> Box<dyn SystemMut> { Box::new(ps) })
    }
}

/// A builder for a new pass of `System`s being added.
///
/// See the `Engine` docs for more information.
#[derive(Debug)]
pub struct EnginePassBuilder<P: SystemMut, B: System> {
    engine: Engine<P>,
    pass: B,
}

impl<P: SystemMut, B: System> EnginePassBuilder<P, B> {
    /// Adds a `System` to be run in parallel with the rest of the pass.
    pub fn add<T: System>(self, system: T) -> EnginePassBuilder<P, Hlist![T, ...B]> {
        EnginePassBuilder {
            engine: self.engine,
            pass: hlist![system, ...self.pass],
        }
    }

    /// Finishes building the pass and adds it to the `Engine`.
    pub fn finish(self) -> Engine<Hlist![Par<B>, ...P]> {
        let EnginePassBuilder { engine, pass } = self;
        engine.map_passes(move |p| hlist![Par(pass), ...p])
    }
}

#[derive(Debug)]
pub struct Mut<T>(T);

#[derive(Debug)]
pub struct Par<T>(T);

impl<H: System, T: SystemMut> SystemMut for Hlist![Par<H>, ...T] {
    fn run(&mut self, cs: &mut ComponentStore) {
        self.tail.run(cs);
        self.head.0.run(cs);
    }
}

impl<H: SystemMut, T: SystemMut> SystemMut for Hlist![Mut<H>, ...T] {
    fn run(&mut self, cs: &mut ComponentStore) {
        self.tail.run(cs);
        self.head.0.run(cs);
    }
}

impl SystemMut for Hlist![] {
    fn run(&mut self, _: &mut ComponentStore) {}
}

impl<H: System, T: System> System for Hlist![H, ...T] {
    fn run(&mut self, cs: &ComponentStore) {
        let h = &mut self.head;
        let t = &mut self.tail;
        let ((), ()) = rayon::join(|| h.run(cs), || t.run(cs));
    }
}

impl System for Hlist![] {
    fn run(&mut self, _: &ComponentStore) {}
}