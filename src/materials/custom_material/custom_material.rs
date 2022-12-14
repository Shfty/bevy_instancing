use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::AlphaMode,
    prelude::{default, AssetServer},
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            AsBindGroup, BindGroup, BindGroupDescriptor, Face, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
        renderer::RenderDevice,
    },
};

use crate::{
    instancing::material::material_instanced::AsBatch,
    prelude::{
        ColorMeshInstance, InstancedMaterialPipeline, MaterialInstanced, CUSTOM_SHADER_HANDLE,
    },
};

#[derive(Debug, Clone, AsBindGroup, TypeUuid)]
#[uuid = "6dc3b9fc-fcfd-4149-8f20-5d3a1573e5da"]
#[bind_group_data(CustomMaterialKey)]
pub struct CustomMaterial {
    pub alpha_mode: AlphaMode,
    pub cull_mode: Option<Face>,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self {
            alpha_mode: default(),
            cull_mode: Some(Face::Back),
        }
    }
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    pub bind_group: BindGroup,
    pub alpha_mode: AlphaMode,
    pub cull_mode: Option<Face>,
}

impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (SRes<RenderDevice>, SRes<InstancedMaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            bind_group,
            alpha_mode: extracted_asset.alpha_mode,
            cull_mode: extracted_asset.cull_mode,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CustomMaterialKey {
    pub cull_mode: Option<Face>,
}

impl PartialOrd for CustomMaterialKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cull_mode
            .map(|cull_mode| cull_mode as usize)
            .partial_cmp(&other.cull_mode.map(|cull_mode| cull_mode as usize))
    }
}

impl Ord for CustomMaterialKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cull_mode
            .map(|cull_mode| cull_mode as usize)
            .cmp(&other.cull_mode.map(|cull_mode| cull_mode as usize))
    }
}

impl From<&CustomMaterial> for CustomMaterialKey {
    fn from(custom_material: &CustomMaterial) -> Self {
        CustomMaterialKey {
            cull_mode: custom_material.cull_mode,
        }
    }
}

impl AsBatch for CustomMaterial {
    type BatchKey = CustomMaterialKey;
}

impl MaterialInstanced for CustomMaterial {
    type Instance = ColorMeshInstance;

    fn vertex_shader(_: &AssetServer) -> ShaderRef {
        CUSTOM_SHADER_HANDLE.typed().into()
    }

    fn fragment_shader(_: &AssetServer) -> ShaderRef {
        CUSTOM_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        _pipeline: &InstancedMaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        key: Self::BatchKey,
        _layout: &MeshVertexBufferLayout,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("custom_{}", *label).into();
        }
        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
