use super::*;

macro_rules! unionize {
  (
    $(#[$s_meta:meta])*
    $s_name:ident {
      $($(#[$f_meta:meta])* $f_name:ident: $s_ty:ty),*
      $(,)?
    }
  ) => {
    $(#[$s_meta])*
    #[repr(C)]
    pub union $s_name {
      $($(#[$f_meta])* pub $f_name: $s_ty),*
    }
    impl Copy for $s_name { }
    impl Clone for $s_name {
      fn clone(&self) -> Self {
        *self
      }
    }
  };
}

unionize! {
  /// [VkAccelerationStructureGeometryDataKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html)
  VkAccelerationStructureGeometryDataKHR {
    /// * **Selection:** VK_GEOMETRY_TYPE_TRIANGLES_KHR
    triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
    /// * **Selection:** VK_GEOMETRY_TYPE_AABBS_KHR
    aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
    /// * **Selection:** VK_GEOMETRY_TYPE_INSTANCES_KHR
    instances: VkAccelerationStructureGeometryInstancesDataKHR,
  }
}

unionize! {
  /// [VkClearColorValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearColorValue.html)
  VkClearColorValue {
    float32: [float; 4],
    int32: [int32_t; 4],
    uint32: [uint32_t; 4],
  }
}

unionize! {
  /// [VkClearValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearValue.html)
  VkClearValue {
    color: VkClearColorValue,
    depthStencil: VkClearDepthStencilValue,
  }
}

unionize! {
  /// [VkDeviceOrHostAddressConstKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressConstKHR.html)
  VkDeviceOrHostAddressConstKHR {
    /// * **No Auto-validity:** true
    deviceAddress: VkDeviceAddress,
    /// * **No Auto-validity:** true
    hostAddress: *const c_void,
  }
}

unionize! {
  /// [VkDeviceOrHostAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressKHR.html)
  VkDeviceOrHostAddressKHR {
    /// * **No Auto-validity:** true
    deviceAddress: VkDeviceAddress,
    /// * **No Auto-validity:** true
    hostAddress: *mut c_void,
  }
}

unionize! {
  /// [VkPerformanceCounterResultKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterResultKHR.html)
  VkPerformanceCounterResultKHR {
    int32: int32_t,
    int64: int64_t,
    uint32: uint32_t,
    uint64: uint64_t,
    float32: float,
    float64: double,
  }
}

unionize! {
  /// [VkPerformanceValueDataINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueDataINTEL.html)
  VkPerformanceValueDataINTEL {
    /// * **Selection:** VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL
    value32: uint32_t,
    /// * **Selection:** VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL
    value64: uint64_t,
    /// * **Selection:** VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL
    valueFloat: float,
    /// * **Selection:** VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL
    valueBool: VkBool32,
    /// * **Len:** null-terminated
    /// * **Selection:** VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL
    valueString: *const u8,
  }
}

unionize! {
  /// [VkPipelineExecutableStatisticValueKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html)
  VkPipelineExecutableStatisticValueKHR {
    /// * **Selection:** VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR
    b32: VkBool32,
    /// * **Selection:** VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR
    i64: int64_t,
    /// * **Selection:** VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR
    u64: uint64_t,
    /// * **Selection:** VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR
    f64: double,
  }
}
