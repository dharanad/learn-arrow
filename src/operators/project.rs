use crate::operators::Operator;
use arrow::array::RecordBatch;
use arrow::datatypes::{Field, Schema, SchemaRef};
use std::sync::Arc;

struct Selection {
    projection: Vec<Field>,
    child: Arc<dyn Operator>,
}

impl Selection {
    pub fn new(projection: Vec<Field>, child: Arc<dyn Operator>) -> Self {
        Self { projection, child }
    }
}

impl Operator for Selection {
    fn name(&self) -> String {
        "Project".to_string()
    }

    fn has_next(&self) -> bool {
        self.child.has_next()
    }

    fn next(&self) -> RecordBatch {
        let child_schema_fields = &self.child.schema().fields;
        let project_indices = child_schema_fields
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if self.projection.contains(f) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let next_batch = self.child.next();
        next_batch.project(&project_indices).unwrap()
    }

    fn schema(&self) -> SchemaRef {
        SchemaRef::new(Schema::new(self.projection.clone()))
    }

    fn child(&self) -> Arc<dyn Operator> {
        Arc::clone(&self.child)
    }
}

#[cfg(test)]
mod tests {
    use crate::operators::project::Selection;
    use crate::operators::scan::{generate_random_batches, Scan};
    use arrow::datatypes::{Field, Schema};
    use std::sync::Arc;
    use crate::operators::Operator;

    #[test]
    fn test_project() {
        // Create 6 fields with different data types
        let fields = vec![
            Field::new("a", arrow::datatypes::DataType::Int32, false),
            Field::new("b", arrow::datatypes::DataType::Int32, false),
            Field::new("c", arrow::datatypes::DataType::Utf8, false),
            Field::new("d", arrow::datatypes::DataType::Utf8, false),
            Field::new("e", arrow::datatypes::DataType::Float32, false),
            Field::new("f", arrow::datatypes::DataType::Float32, false),
        ];

        let data = generate_random_batches(fields.clone(), 1000, 1000);
        let schema = Schema::new(fields);
        let scan = Scan::new(data, schema);

        let project = Selection::new(
            vec![
                Field::new("a", arrow::datatypes::DataType::Int32, false),
                Field::new("c", arrow::datatypes::DataType::Utf8, false),
                Field::new("e", arrow::datatypes::DataType::Float32, false),
            ],
            Arc::new(scan),
        );

        // execute
        while project.has_next() {
            let data = project.next();
            assert_eq!(data.schema().fields.len(), 3);
        }
    }
}
