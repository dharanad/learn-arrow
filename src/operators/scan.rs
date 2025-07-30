use std::{cell::RefCell, sync::Arc};

use arrow::{array::RecordBatch, datatypes::Schema};
use arrow::array::{ArrayRef, Float32Array, Int32Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field};
use rand::{thread_rng, Rng};
use crate::operators::Operator;

pub struct Scan {
    ptr: RefCell<usize>,
    data: Vec<RecordBatch>,
    schema: Schema
}

impl Scan {
    pub fn new(data: Vec<RecordBatch>, schema: Schema) -> Self {
        Self {
            ptr: RefCell::new(0),
            data,
            schema
        }
    }
}

impl Operator for Scan {
    fn name(&self) -> String {
        return "Scan".to_string();
    }

    fn has_next(&self) -> bool {
        let curr = * self.ptr.borrow();
        curr < self.data.len()
    }

    fn next(&self) -> RecordBatch {
        let curr = * self.ptr.borrow();
        let n = &self.data[curr];
        *self.ptr.borrow_mut() += 1;
        n.clone()
    }

    fn schema(&self) -> arrow::datatypes::SchemaRef {
        let val = self.schema.clone();
        Arc::new(val)
    }

    fn child(&self) -> Arc<dyn Operator> {
        panic!("{} is leaf operator", self.name())
    }
}
pub fn generate_random_batches(
    fields: Vec<Field>,
    batch_size: usize,
    num_batches: usize,
) -> Vec<RecordBatch> {
    let schema = Arc::new(Schema::new(fields));
    let mut batches = Vec::with_capacity(num_batches);
    let mut rng = thread_rng();

    for _ in 0..num_batches {
        let arrays: Vec<ArrayRef> = schema
            .fields()
            .iter()
            .map(|field| match field.data_type() {
                DataType::Int32 => {
                    let arr: Int32Array = (0..batch_size)
                        .map(|_| rng.gen_range(-1000..1000))
                        .collect();
                    Arc::new(arr)
                }
                DataType::Float32 => {
                    let arr = (0..batch_size)
                        .map(|_| rng.gen_range(-1000.0..1000.0))
                        .collect::<Vec<_>>();
                    Arc::new(Float32Array::from(arr)) as ArrayRef
                }
                DataType::Utf8 => {
                    let arr = (0..batch_size)
                        .map(|i| format!("str_{}", i))
                        .collect::<Vec<_>>();
                    Arc::new(StringArray::from(arr)) as ArrayRef
                }
                _ => panic!("Unsupported data type: {}", field.data_type()),
            })
            .collect();

        let batch = RecordBatch::try_new(schema.clone(), arrays)
            .expect("Failed to create record batch");
        batches.push(batch);
    }

    batches
}

#[cfg(test)]
mod tests {
    use arrow::datatypes::{DataType, Field, Schema};
    use crate::operators::Operator;
    use crate::operators::scan::{generate_random_batches, Scan};

    #[test]
    pub fn test_scan() {
        let fields = vec![
            Field::new("a", DataType::Int32, false),
            Field::new("b", DataType::Float32, false),
            Field::new("c", DataType::Utf8, false),
        ];
        let batch_data = generate_random_batches(fields.clone(), 10, 100);
        let scan = Scan::new(batch_data, Schema::new(fields));

        // assert batch count == 100
        let mut count = 0;
        while scan.has_next() {
            let batch  = scan.next();
            assert_eq!(batch.schema(), scan.schema());
            count += 1;
        }
        assert_eq!(count, 100);

    }
}