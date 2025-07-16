fn main() {
    assigment_1::three();
}

#[allow(dead_code)]
#[allow(unused_variables)]
mod assigment_1 {

    use std::sync::Arc;

    use arrow::{
        array::{Array, ArrayRef, AsArray, Decimal128Array, Float32Array, Int32Array, Int64Array, RecordBatch, StringArray},
        datatypes::{DataType, Field, Int64Type, Schema},
    };

    pub fn one() {
        // Creating Int32 Array
        let a_i32: arrow::array::PrimitiveArray<arrow::datatypes::Int32Type> =
            Int32Array::from(vec![1, 2, 3, 4, 5, 6]);
        println!("{:?}", a_i32);
        // Create Int32 Array using builder
        let mut b_builder = Int32Array::builder(15);
        b_builder.append_value(1);
        b_builder.append_value(2);
        b_builder.append_null();
        b_builder.append_value_n(3, 3);
        b_builder.append_null();
        b_builder.append_value_n(4, 5);
        let b_i32: arrow::array::PrimitiveArray<arrow::datatypes::Int32Type> = b_builder.finish();
        println!("{:?}", b_i32);
        assert_eq!(b_i32.data_type().clone(), DataType::Int32);
        assert_eq!(b_i32.null_count(), 2);
        assert_eq!(b_i32.len(), 12);
        assert!(b_i32.is_null(2));

        // Create Float32 Array
        let c_f32 = Float32Array::from(vec![1.2, 2.1, 5.5]);
        println!("{:?}", c_f32);
        assert_eq!(c_f32.data_type().clone(), DataType::Float32);
        assert_eq!(c_f32.null_count(), 0);

        // Create String Array
        let d_string: arrow::array::GenericByteArray<arrow::datatypes::GenericStringType<i32>> =
            StringArray::from(vec!["Dharan", "Aditya", "Guthula"]);
        println!("{:?}", d_string);
        assert_eq!(d_string.data_type().clone(), DataType::Utf8);
        assert_eq!(d_string.is_nullable(), false);
        assert!(!d_string.is_null(0));
        assert_eq!(d_string.value(0), "Dharan");
    }

    fn pretty_print(arr_ref: ArrayRef) {
        println!("Data Type: {}", arr_ref.data_type());
        println!("Length: {}", arr_ref.len());
        println!("Null Count: {}", arr_ref.null_count());
        println!("{:?}", arr_ref)
    }

    pub fn two() {
        // empty schema
        let empty_schema = Schema::empty();

        /*
        ORDERS
        +-----------------+-------------------+-------------+
        | column_name     | data_type         | is_nullable |
        +-----------------+-------------------+-------------+
        | o_orderkey      | Int64             | NO          |
        | o_custkey       | Int64             | NO          |
        | o_orderstatus   | Utf8View          | NO          |
        | o_totalprice    | Decimal128(15, 2) | NO          |
        | o_orderdate     | Date32            | NO          |
        | o_orderpriority | Utf8View          | NO          |
        | o_clerk         | Utf8View          | NO          |
        | o_shippriority  | Int32             | NO          |
        | o_comment       | Utf8View          | NO          |
        +-----------------+-------------------+-------------+d
        */

        let orders_schema = Schema::new(vec![
            Field::new("o_orderkey", DataType::Int64, false),
            Field::new("o_custkey", DataType::Int64, false),
            Field::new("o_orderstatus", DataType::Utf8, false),
            Field::new("o_totalprice", DataType::Decimal128(15, 2), false),
            Field::new("o_orderdate", DataType::Date32, false),
            Field::new("o_orderpriority", DataType::Utf8, false),
            Field::new("o_clerk", DataType::Utf8, false),
            Field::new("o_shippriority", DataType::Int32, false),
            Field::new("o_comment", DataType::Utf8, false),
        ]);
        println!("Schema {:?}", orders_schema);
        assert_eq!(orders_schema.fields().len(), 9);
        assert!(orders_schema.field_with_name("o_orderkey").is_ok());
        assert!(orders_schema.field_with_name("l_orderkey").is_err());
        assert_eq!(
            orders_schema
                .field_with_name("o_orderkey")
                .unwrap()
                .data_type(),
            &DataType::Int64
        );

        println!();

        let new_schema = orders_schema.project(&[0, 6, 8]);
        assert!(new_schema.is_ok());
        let new_schema = new_schema.unwrap();
        assert_eq!(new_schema.fields().get(0).unwrap().name(), "o_orderkey");
        assert_eq!(new_schema.fields().get(1).unwrap().name(), "o_clerk");
        assert_eq!(new_schema.fields().get(2).unwrap().name(), "o_comment");

        assert_eq!(
            new_schema.fields().get(0).unwrap().data_type(),
            &DataType::Int64
        );
        assert_eq!(
            new_schema.fields().get(1).unwrap().data_type(),
            &DataType::Utf8
        );
        assert_eq!(
            new_schema.fields().get(2).unwrap().data_type(),
            &DataType::Utf8
        );
    }

    pub fn three() {
        /*

        > describe customer;
        +--------------+-------------------+-------------+
        | column_name  | data_type         | is_nullable |
        +--------------+-------------------+-------------+
        | c_custkey    | Int64             | NO          |
        | c_name       | Utf8View          | NO          |
        | c_address    | Utf8View          | NO          |
        | c_nationkey  | Int64             | NO          |
        | c_phone      | Utf8View          | NO          |
        | c_acctbal    | Decimal128(15, 2) | NO          |
        | c_mktsegment | Utf8View          | NO          |
        | c_comment    | Utf8View          | NO          |
        +--------------+-------------------+-------------+
        */

        let customer_schema = Schema::new(vec![
            Field::new("c_custkey", DataType::Int64, false),
            Field::new("c_name", DataType::Utf8, false),
            Field::new("c_address", DataType::Utf8, false),
            Field::new("c_nationkey", DataType::Int64, false),
            Field::new("c_acctbal", DataType::Decimal128(15, 2), false),
        ]);

        let custkey = Int64Array::from(vec![25062, 25063, 25064, 25065]);
        let name = StringArray::from(vec!["Customer#000025062", "Customer#000025063", "Customer#000025064", "Customer#000025065"]);
        let address = StringArray::from(vec!["PodqaseGMDrG", "ycFfPCs0iXRVmOspKO7OQOx", "k0rDE0jHbR", "qNkYPUKZtk"]);
        let nation_key = Int64Array::from(vec![12, 17, 16, 3]);
        let accbal = Decimal128Array::from(vec![31557, -25378, 62509, 990228]).with_precision_and_scale(15, 2).unwrap();

        let record_batch = RecordBatch::try_new(Arc::new(customer_schema), vec![
            Arc::new(custkey),
            Arc::new(name),
            Arc::new(address),
            Arc::new(nation_key),
            Arc::new(accbal)
        ]);
        assert!(record_batch.is_ok());
        let record_batch = record_batch.unwrap();

        assert_eq!(record_batch.num_columns(), 5);
        assert_eq!(record_batch.num_rows(), 4);

        
        let custkey_col = record_batch.column_by_name("c_custkey");
        assert!(custkey_col.is_some());
        let custkey_col = custkey_col.unwrap();
        assert_eq!(custkey_col.data_type(), &DataType::Int64);
        let downcasted_array = custkey_col.as_any().downcast_ref::<Int64Array>().unwrap();
        assert_eq!(downcasted_array.value(1), 25063);
    }
}
