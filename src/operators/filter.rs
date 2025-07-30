use std::sync::Arc;
use arrow::array::RecordBatch;
use arrow::datatypes::{Field, SchemaRef};
use crate::operators::Operator;

struct Filter {
    child: Arc<dyn Operator>,
    filter: Field
}

impl Operator for Filter {
    fn name(&self) -> String {
        "Filter".to_string()
    }

    fn has_next(&self) -> bool {
        self.child.has_next()
    }

    fn next(&self) -> RecordBatch {
        let data = self.child.next();
        data.
    }

    fn schema(&self) -> SchemaRef {
        self.child.schema()
    }

    fn child(&self) -> Arc<dyn Operator> {
        Arc::clone(&self.child)
    }
}