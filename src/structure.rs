#[derive(Clone, Debug)]
pub struct Row<T> {
    pub cells: Vec<T>
}

#[derive(Debug)]
pub struct Grid<T> {
    pub rows: Vec<Row<T>>
}
