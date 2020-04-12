use crate::{
    draw_target::DrawTarget,
    mesh::{Mesh, MeshType},
    pipeline::PipelineDescriptor,
    render_resource::{
        resource_name, BufferInfo, BufferUsage, RenderResource, RenderResourceAssignments,
        ResourceInfo,
    },
    renderer::{RenderPass, Renderer},
};
use bevy_asset::{Asset, Handle};
use legion::prelude::*;

use zerocopy::AsBytes;

#[derive(Default)]
pub struct UiDrawTarget {
    pub mesh_vertex_buffer: Option<RenderResource>,
    pub mesh_index_buffer: Option<RenderResource>,
    pub mesh_index_length: usize,
    pub mesh: Option<Handle<Mesh>>,
}

impl DrawTarget for UiDrawTarget {
    fn draw(
        &self,
        _world: &World,
        resources: &Resources,
        render_pass: &mut dyn RenderPass,
        _pipeline_handle: Handle<PipelineDescriptor>,
    ) {
        let render_resource_assignments = resources.get::<RenderResourceAssignments>().unwrap();
        let ui_instances_buffer = {
            match render_resource_assignments.get(resource_name::buffer::UI_INSTANCES) {
                Some(buffer) => buffer,
                None => return,
            }
        };

        let index_count = {
            let renderer = render_pass.get_renderer();
            if let Some(ResourceInfo::Buffer(BufferInfo {
                array_info: Some(array_info),
                ..
            })) = renderer.get_resource_info(ui_instances_buffer)
            {
                Some(array_info.item_capacity)
            } else {
                None
            }
        };

        let global_render_resource_assignments =
            resources.get::<RenderResourceAssignments>().unwrap();
        render_pass.set_render_resources(&global_render_resource_assignments);
        render_pass.set_index_buffer(self.mesh_index_buffer.unwrap(), 0);
        render_pass.set_vertex_buffer(0, self.mesh_vertex_buffer.unwrap(), 0);
        render_pass.set_vertex_buffer(1, ui_instances_buffer, 0);
        render_pass.draw_indexed(
            0..self.mesh_index_length as u32,
            0,
            0..(index_count.unwrap() as u32),
        );
    }

    fn setup(
        &mut self,
        _world: &World,
        resources: &Resources,
        renderer: &mut dyn Renderer,
        _pipeline_handle: Handle<PipelineDescriptor>,
        pipeline_descriptor: &PipelineDescriptor,
    ) {
        // don't create meshes if they have already been created
        if let Some(_) = self.mesh_vertex_buffer {
            return;
        }

        let quad = Mesh::load(MeshType::Quad {
            size: glam::vec2(1.0, 1.0),
        });
        self.mesh_vertex_buffer = Some(renderer.create_buffer_with_data(
            BufferInfo {
                buffer_usage: BufferUsage::VERTEX,
                ..Default::default()
            },
            quad.vertices.as_bytes(),
        ));
        self.mesh_index_buffer = Some(renderer.create_buffer_with_data(
            BufferInfo {
                buffer_usage: BufferUsage::INDEX,
                ..Default::default()
            },
            quad.indices.as_bytes(),
        ));
        self.mesh_index_length = quad.indices.len();

        let mut global_render_resource_assignments =
            resources.get_mut::<RenderResourceAssignments>().unwrap();
        renderer.setup_bind_groups(&mut global_render_resource_assignments, pipeline_descriptor);
    }
    fn get_name(&self) -> String {
        resource_name::draw_target::UI.to_string()
    }
}