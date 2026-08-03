//! Pure reference model for M6 dual-layout table semantics.
//!
//! Independent of the product compiler: proves that `rows` and `columns`
//! store the same logical cells under the published address formulas.

const MAX_CAPACITY: usize = 1_024;
const MAX_FIELDS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Layout {
    Rows,
    Columns,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Shape {
    fields: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Table {
    shape: Shape,
    layout: Layout,
    capacity: usize,
    cells: Vec<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelError {
    IllegalShape,
    UnknownField,
    CapacityBound,
    IndexBound,
}

fn cell_address(
    layout: Layout,
    capacity: usize,
    field_count: usize,
    index: usize,
    field: usize,
) -> usize {
    match layout {
        Layout::Rows => index * field_count + field,
        Layout::Columns => field * capacity + index,
    }
}

fn make_shape(fields: &[&'static str]) -> Result<Shape, ModelError> {
    if fields.is_empty() || fields.len() > MAX_FIELDS {
        return Err(ModelError::IllegalShape);
    }
    let mut seen = Vec::new();
    for field in fields {
        if seen.contains(field) {
            return Err(ModelError::IllegalShape);
        }
        seen.push(*field);
    }
    Ok(Shape {
        fields: fields.to_vec(),
    })
}

fn allocate(shape: Shape, layout: Layout, capacity: usize) -> Result<Table, ModelError> {
    if !(1..=MAX_CAPACITY).contains(&capacity) {
        return Err(ModelError::CapacityBound);
    }
    let cells = vec![0; capacity * shape.fields.len()];
    Ok(Table {
        shape,
        layout,
        capacity,
        cells,
    })
}

fn field_index(shape: &Shape, field: &str) -> Result<usize, ModelError> {
    shape
        .fields
        .iter()
        .position(|candidate| *candidate == field)
        .ok_or(ModelError::UnknownField)
}

fn store(table: &mut Table, index: usize, field: &str, value: i64) -> Result<(), ModelError> {
    if index >= table.capacity {
        return Err(ModelError::IndexBound);
    }
    let field = field_index(&table.shape, field)?;
    let address = cell_address(
        table.layout,
        table.capacity,
        table.shape.fields.len(),
        index,
        field,
    );
    table.cells[address] = value;
    Ok(())
}

fn load(table: &Table, index: usize, field: &str) -> Result<i64, ModelError> {
    if index >= table.capacity {
        return Err(ModelError::IndexBound);
    }
    let field = field_index(&table.shape, field)?;
    let address = cell_address(
        table.layout,
        table.capacity,
        table.shape.fields.len(),
        index,
        field,
    );
    Ok(table.cells[address])
}

fn run_script(layout: Layout) -> Result<Vec<i64>, ModelError> {
    let shape = make_shape(&["mass", "charge"])?;
    let mut table = allocate(shape, layout, 4)?;
    store(&mut table, 0, "mass", 10)?;
    store(&mut table, 0, "charge", 3)?;
    store(&mut table, 1, "mass", 7)?;
    store(&mut table, 1, "charge", 2)?;
    store(&mut table, 3, "mass", 1)?;
    Ok(vec![
        load(&table, 0, "mass")?,
        load(&table, 0, "charge")?,
        load(&table, 1, "mass")?,
        load(&table, 1, "charge")?,
        load(&table, 3, "mass")?,
        load(&table, 2, "charge")?,
    ])
}

#[test]
fn dual_layout_scripts_are_observationally_equivalent() {
    let rows = run_script(Layout::Rows).expect("rows script");
    let columns = run_script(Layout::Columns).expect("columns script");
    assert_eq!(rows, columns);
    assert_eq!(rows, vec![10, 3, 7, 2, 1, 0]);
}

#[test]
fn rejects_illegal_shapes_fields_and_capacity() {
    assert_eq!(make_shape(&[]), Err(ModelError::IllegalShape));
    assert_eq!(
        make_shape(&["a", "b", "c", "d", "e", "f", "g", "h", "i"]),
        Err(ModelError::IllegalShape)
    );
    assert_eq!(make_shape(&["mass", "mass"]), Err(ModelError::IllegalShape));
    let shape = make_shape(&["mass"]).expect("shape");
    assert_eq!(
        allocate(shape.clone(), Layout::Rows, 0),
        Err(ModelError::CapacityBound)
    );
    assert_eq!(
        allocate(shape, Layout::Rows, MAX_CAPACITY + 1),
        Err(ModelError::CapacityBound)
    );
    let mut table =
        allocate(make_shape(&["mass"]).expect("shape"), Layout::Rows, 1).expect("allocate");
    assert_eq!(
        store(&mut table, 0, "charge", 1),
        Err(ModelError::UnknownField)
    );
    assert_eq!(store(&mut table, 1, "mass", 1), Err(ModelError::IndexBound));
}

#[test]
fn physical_addresses_differ_while_logical_cells_match() {
    let shape = make_shape(&["mass", "charge"]).expect("shape");
    assert_eq!(cell_address(Layout::Rows, 4, 2, 1, 1), 3);
    assert_eq!(cell_address(Layout::Columns, 4, 2, 1, 1), 5);
    let mut rows = allocate(shape.clone(), Layout::Rows, 4).expect("rows");
    let mut columns = allocate(shape, Layout::Columns, 4).expect("columns");
    store(&mut rows, 1, "charge", 42).expect("store rows");
    store(&mut columns, 1, "charge", 42).expect("store columns");
    assert_ne!(rows.cells, columns.cells);
    assert_eq!(load(&rows, 1, "charge").expect("load rows"), 42);
    assert_eq!(load(&columns, 1, "charge").expect("load columns"), 42);
}
