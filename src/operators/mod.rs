use std::sync::Arc;

use arrow::{array::RecordBatch, datatypes::SchemaRef};

mod scan;
mod project;
mod filter;

pub trait Operator {
    fn name(&self) -> String;
    fn has_next(&self) -> bool;
    fn next(&self) -> RecordBatch;
    fn schema(&self) -> SchemaRef;
    fn child(&self) -> Arc<dyn Operator>;
}