use crate::{object_id::ObjectId, ContextProxy};
use std::any::Any;

pub trait Controller: Any {
    fn on_ready(&mut self, object_id: ObjectId, ctx: &mut ContextProxy);
    fn on_destroy(&mut self, object_id: ObjectId, ctx: &mut ContextProxy);
    fn on_update(&mut self, object_id: ObjectId, ctx: &mut ContextProxy);
    fn on_late_update(&mut self, object_id: ObjectId, ctx: &mut ContextProxy);
}