use std::sync::Arc;

use vulkano::command_buffer::allocator::{CommandBufferAllocator, StandardCommandBufferAllocator};
use vulkano::descriptor_set::allocator::{DescriptorSetAllocator, StandardDescriptorSetAlloc, StandardDescriptorSetAllocator};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::device::{Device, DeviceOwned};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::OomError;

pub struct MemoryAllocator {
    device: Arc<Device>,
    memory_allocator: StandardMemoryAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
    command_buffer_allocator: StandardCommandBufferAllocator,
}
impl MemoryAllocator {
    pub fn new(device: Arc<Device>) -> Self {
        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
        let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());

        Self {
            device,
            memory_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
        }
    }
}

unsafe impl DeviceOwned for MemoryAllocator {
    fn device(&self) -> &Arc<vulkano::device::Device> {
        &self.device
    }
}

unsafe impl vulkano::memory::allocator::MemoryAllocator for MemoryAllocator {
    fn find_memory_type_index(&self, memory_type_bits: u32, filter: vulkano::memory::allocator::MemoryTypeFilter) -> Option<u32> {
        self.memory_allocator.find_memory_type_index(memory_type_bits, filter)
    }

    fn allocate_from_type(
        &self,
        memory_type_index: u32,
        create_info: vulkano::memory::allocator::SuballocationCreateInfo,
    ) -> Result<vulkano::memory::allocator::MemoryAlloc, vulkano::memory::allocator::AllocationCreationError> {
        self.memory_allocator.allocate_from_type(memory_type_index, create_info)
    }

    unsafe fn allocate_from_type_unchecked(
        &self,
        memory_type_index: u32,
        create_info: vulkano::memory::allocator::SuballocationCreateInfo,
        never_allocate: bool,
    ) -> Result<vulkano::memory::allocator::MemoryAlloc, vulkano::memory::allocator::AllocationCreationError> {
        self.memory_allocator
            .allocate_from_type_unchecked(memory_type_index, create_info, never_allocate)
    }

    fn allocate(
        &self,
        create_info: vulkano::memory::allocator::AllocationCreateInfo<'_>,
    ) -> Result<vulkano::memory::allocator::MemoryAlloc, vulkano::memory::allocator::AllocationCreationError> {
        self.memory_allocator.allocate(create_info)
    }

    unsafe fn allocate_unchecked(
        &self,
        create_info: vulkano::memory::allocator::AllocationCreateInfo<'_>,
    ) -> Result<vulkano::memory::allocator::MemoryAlloc, vulkano::memory::allocator::AllocationCreationError> {
        self.memory_allocator.allocate_unchecked(create_info)
    }

    unsafe fn allocate_dedicated_unchecked(
        &self,
        memory_type_index: u32,
        allocation_size: vulkano::DeviceSize,
        dedicated_allocation: Option<vulkano::memory::DedicatedAllocation<'_>>,
        export_handle_types: vulkano::memory::ExternalMemoryHandleTypes,
    ) -> Result<vulkano::memory::allocator::MemoryAlloc, vulkano::memory::allocator::AllocationCreationError> {
        self.memory_allocator
            .allocate_dedicated_unchecked(memory_type_index, allocation_size, dedicated_allocation, export_handle_types)
    }
}

unsafe impl DescriptorSetAllocator for MemoryAllocator {
    type Alloc = <StandardDescriptorSetAllocator as DescriptorSetAllocator>::Alloc;

    fn allocate(&self, layout: &Arc<DescriptorSetLayout>, variable_descriptor_count: u32) -> Result<StandardDescriptorSetAlloc, OomError> {
        self.descriptor_set_allocator.allocate(layout, variable_descriptor_count)
    }
}

unsafe impl CommandBufferAllocator for MemoryAllocator {
    type Alloc = <StandardCommandBufferAllocator as CommandBufferAllocator>::Alloc;
    type Builder = <StandardCommandBufferAllocator as CommandBufferAllocator>::Builder;
    type Iter = <StandardCommandBufferAllocator as CommandBufferAllocator>::Iter;

    fn allocate(
        &self,
        queue_family_index: u32,
        level: vulkano::command_buffer::CommandBufferLevel,
        command_buffer_count: u32,
    ) -> Result<Self::Iter, OomError> {
        self.command_buffer_allocator
            .allocate(queue_family_index, level, command_buffer_count)
    }
}
