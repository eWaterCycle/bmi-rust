#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// A BMI operation's result, preserving the model's error information.
pub type BmiResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// The location of a variable's values on its grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// Grid nodes.
    Node,
    /// Grid edges.
    Edge,
    /// Grid faces.
    Face,
}

/// Grid types defined by BMI 2.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridType {
    /// A single value with no spatial dimensions.
    Scalar,
    /// Disconnected points.
    Points,
    /// A one-dimensional unstructured grid.
    Vector,
    /// An unstructured grid with explicit connectivity.
    Unstructured,
    /// A structured grid of quadrilateral cells.
    StructuredQuadrilateral,
    /// A structured grid with independently varying axis spacing.
    Rectilinear,
    /// A structured grid with constant spacing along each axis.
    UniformRectilinear,
}

/// An owned copy of a variable's numeric values, flattened in model order.
#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    /// Signed 8-bit integers.
    I8(Vec<i8>),
    /// Unsigned 8-bit integers.
    U8(Vec<u8>),
    /// Signed 16-bit integers.
    I16(Vec<i16>),
    /// Unsigned 16-bit integers.
    U16(Vec<u16>),
    /// Signed 32-bit integers.
    I32(Vec<i32>),
    /// Unsigned 32-bit integers.
    U32(Vec<u32>),
    /// Signed 64-bit integers.
    I64(Vec<i64>),
    /// Unsigned 64-bit integers.
    U64(Vec<u64>),
    /// 32-bit floating-point values.
    F32(Vec<f32>),
    /// 64-bit floating-point values.
    F64(Vec<f64>),
}

/// Borrowed numeric values used for direct reads and setter inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RefValues<'a> {
    /// Signed 8-bit integers.
    I8(&'a [i8]),
    /// Unsigned 8-bit integers.
    U8(&'a [u8]),
    /// Signed 16-bit integers.
    I16(&'a [i16]),
    /// Unsigned 16-bit integers.
    U16(&'a [u16]),
    /// Signed 32-bit integers.
    I32(&'a [i32]),
    /// Unsigned 32-bit integers.
    U32(&'a [u32]),
    /// Signed 64-bit integers.
    I64(&'a [i64]),
    /// Unsigned 64-bit integers.
    U64(&'a [u64]),
    /// 32-bit floating-point values.
    F32(&'a [f32]),
    /// 64-bit floating-point values.
    F64(&'a [f64]),
}

/// The [CSDMS Basic Model Interface 2.0](https://bmi.csdms.io/en/stable/).
///
/// Initialize a model before querying or updating it, and finalize it after use.
/// Variable names follow the model's advertised input and output names. Grid
/// identifiers are model-defined; indices and sizes use Rust's `usize`.
///
/// All methods are fallible and required. Implementations should return errors
/// for invalid names, grid identifiers, indices, types, buffer lengths, lifecycle
/// states, and operations unsupported by their grid or storage representation.
/// Setters must validate their inputs before modifying model storage.
///
/// This trait is dyn-compatible, so coupling code can use `Box<dyn Bmi>`.
pub trait Bmi {
    /// Load configuration from `config_file` and allocate model resources.
    /// An empty path requests the model's default configuration, if supported.
    fn initialize(&mut self, config_file: &str) -> BmiResult<()>;
    /// Advance the model by one time step.
    fn update(&mut self) -> BmiResult<()>;
    /// Advance to `time`, expressed in the model's time units.
    fn update_until(&mut self, time: f64) -> BmiResult<()>;
    /// Release model resources and finish any pending output.
    fn finalize(&mut self) -> BmiResult<()>;

    /// Return the model's human-readable component name.
    fn get_component_name(&self) -> BmiResult<String>;
    /// Return the number of input variables.
    fn get_input_item_count(&self) -> BmiResult<usize>;
    /// Return the number of output variables.
    fn get_output_item_count(&self) -> BmiResult<usize>;
    /// Return the names of variables that can be set.
    fn get_input_var_names(&self) -> BmiResult<Vec<String>>;
    /// Return the names of variables that can be read.
    fn get_output_var_names(&self) -> BmiResult<Vec<String>>;

    /// Return the grid identifier associated with a variable.
    fn get_var_grid(&self, name: &str) -> BmiResult<i32>;
    /// Return the variable's native type name (for example, `float64`).
    fn get_var_type(&self, name: &str) -> BmiResult<String>;
    /// Return the variable's units, preferably using UDUNITS conventions.
    fn get_var_units(&self, name: &str) -> BmiResult<String>;
    /// Return the size of one variable element in bytes.
    fn get_var_itemsize(&self, name: &str) -> BmiResult<usize>;
    /// Return the total size of a variable's values in bytes.
    fn get_var_nbytes(&self, name: &str) -> BmiResult<usize>;
    /// Return whether the variable is defined on nodes, edges, or faces.
    fn get_var_location(&self, name: &str) -> BmiResult<Location>;

    /// Return the current simulation time.
    fn get_current_time(&self) -> BmiResult<f64>;
    /// Return the simulation's start time.
    fn get_start_time(&self) -> BmiResult<f64>;
    /// Return the simulation's end time.
    fn get_end_time(&self) -> BmiResult<f64>;
    /// Return the units used by every time query and `update_until`.
    fn get_time_units(&self) -> BmiResult<String>;
    /// Return the model's current time-step duration.
    fn get_time_step(&self) -> BmiResult<f64>;

    /// Return an owned copy of all values, preserving their numeric type.
    fn get_value(&self, name: &str) -> BmiResult<Values>;
    /// Borrow the model's storage without copying it.
    ///
    /// The slice cannot outlive the model borrow. Rust prevents mutable calls,
    /// including `update`, `set_value`, and `finalize`, while it is still in use.
    /// Changes must go through the setters; no mutable raw pointer is exposed.
    fn get_value_ptr(&self, name: &str) -> BmiResult<RefValues<'_>>;
    /// Copy values at zero-based flattened indices, in the requested order.
    /// Repeated indices repeat values; an empty index slice returns no values.
    fn get_value_at_indices(&self, name: &str, indices: &[usize]) -> BmiResult<Values>;
    /// Copy `src` into a variable. Its type and length must match the variable.
    fn set_value(&mut self, name: &str, src: RefValues<'_>) -> BmiResult<()>;
    /// Copy values into zero-based flattened indices.
    /// `src` must match the variable's type and have one value per index.
    fn set_value_at_indices(
        &mut self,
        name: &str,
        indices: &[usize],
        src: RefValues<'_>,
    ) -> BmiResult<()>;

    /// Return the number of spatial dimensions.
    fn get_grid_rank(&self, grid: i32) -> BmiResult<usize>;
    /// Return the total number of grid nodes.
    fn get_grid_size(&self, grid: i32) -> BmiResult<usize>;
    /// Return the grid's topology and geometry category.
    fn get_grid_type(&self, grid: i32) -> BmiResult<GridType>;
    /// Return the node count along each axis, ordered slowest to fastest (z, y, x).
    /// The returned length equals the grid rank.
    fn get_grid_shape(&self, grid: i32) -> BmiResult<Vec<usize>>;
    /// Return uniform axis spacing, ordered as `get_grid_shape`.
    fn get_grid_spacing(&self, grid: i32) -> BmiResult<Vec<f64>>;
    /// Return the grid origin, ordered as `get_grid_shape`.
    fn get_grid_origin(&self, grid: i32) -> BmiResult<Vec<f64>>;
    /// Return x coordinates: axis coordinates for rectilinear grids, otherwise per node.
    fn get_grid_x(&self, grid: i32) -> BmiResult<Vec<f64>>;
    /// Return y coordinates: axis coordinates for rectilinear grids, otherwise per node.
    fn get_grid_y(&self, grid: i32) -> BmiResult<Vec<f64>>;
    /// Return z coordinates: axis coordinates for rectilinear grids, otherwise per node.
    fn get_grid_z(&self, grid: i32) -> BmiResult<Vec<f64>>;
    /// Return the number of nodes in an unstructured grid.
    fn get_grid_node_count(&self, grid: i32) -> BmiResult<usize>;
    /// Return the number of edges in an unstructured grid.
    fn get_grid_edge_count(&self, grid: i32) -> BmiResult<usize>;
    /// Return the number of faces in an unstructured grid.
    fn get_grid_face_count(&self, grid: i32) -> BmiResult<usize>;
    /// Return flattened node pairs for edges, with two indices per edge.
    fn get_grid_edge_nodes(&self, grid: i32) -> BmiResult<Vec<usize>>;
    /// Return flattened edge indices for faces, grouped in face order.
    fn get_grid_face_edges(&self, grid: i32) -> BmiResult<Vec<usize>>;
    /// Return flattened node indices for faces, grouped in face order.
    fn get_grid_face_nodes(&self, grid: i32) -> BmiResult<Vec<usize>>;
    /// Return the node count per face, delimiting the flattened face connectivity arrays.
    fn get_grid_nodes_per_face(&self, grid: i32) -> BmiResult<Vec<usize>>;
}
