initSidebarItems({"enum":[["FramebufferCreationError","Error that can happen when creating a framebuffer object."],["IncompatibleRenderPassAttachmentError","Error that can happen when an image is not compatible with a render pass attachment slot."],["LoadOp","Describes what the implementation should do with an attachment at the start of the subpass."],["RenderPassCreationError","Error that can happen when creating a compute pipeline."],["StoreOp","Describes what the implementation should do with an attachment after all the subpasses have completed."],["SubpassContents","Describes what a subpass in a command buffer will contain."]],"fn":[["ensure_image_view_compatible","Checks whether the given image view is allowed to be the nth attachment of the given render pass."]],"struct":[["AttachmentDescription","Describes an attachment that will be used in a render pass."],["EmptySinglePassRenderPassDesc","Description of an empty render pass."],["Framebuffer","Contains a render pass and the image views that are attached to it."],["FramebufferBuilder","Prototype of a framebuffer."],["FramebufferSys","Opaque object that represents the internals of a framebuffer."],["PassDependencyDescription","Describes a dependency between two passes of a render pass."],["PassDescription","Describes one of the passes of a render pass."],["RenderPass","Defines the layout of multiple subpasses."],["RenderPassDescAttachments","Iterator to the attachments of a `RenderPassDesc`."],["RenderPassDescDependencies","Iterator to the subpass dependencies of a `RenderPassDesc`."],["RenderPassDescSubpasses","Iterator to the subpasses of a `RenderPassDesc`."],["RenderPassSys","Opaque object that represents the render pass' internals."],["Subpass","Represents a subpass within a `RenderPassAbstract` object."]],"trait":[["AttachmentsList","A list of attachments."],["FramebufferAbstract","Trait for objects that contain a Vulkan framebuffer object."],["RenderPassAbstract","Trait for objects that contain a Vulkan render pass object."],["RenderPassCompatible","Trait implemented on render pass objects to check whether they are compatible with another render pass."],["RenderPassDesc","Trait for objects that contain the description of a render pass."],["RenderPassDescClearValues","Extension trait for `RenderPassDesc`. Defines which types are allowed as a list of clear values."],["RenderPassSubpassInterface","Extension trait for `RenderPassDesc` that checks whether a subpass of this render pass accepts the output of a fragment shader."]]});