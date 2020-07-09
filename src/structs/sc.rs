use num_traits::Num;

#[derive(Debug)]
pub struct CcsMatrix<T: Num + Copy + std::fmt::Debug> {
    capability: usize,
    non_zeros: usize,
    row: usize,
    col: usize,
    col_ptr: Vec<usize>,
    row_index: Vec<usize>,
    values: Vec<T>
}

impl <T: Num + Copy + std::fmt::Debug> CcsMatrix<T> {
    pub fn to_column_order(v: &Vec<T>, row: usize, col: usize) -> Vec<T> {
        (0..col).flat_map(|i| {
            (0..row).map(move |j| {
                v[i + j * row]
            })
        }).collect()
    }

    pub fn new(v: &Vec<T>, row: usize, col: usize) -> Self {
        let capability: usize = row * col; println!("{:?}", v.len());
        assert_eq!(v.len(), capability);
        let reorder_v = Self::to_column_order(v, row, col);
        let mut values: Vec<T> = Vec::new();
        let mut row_index: Vec<usize> = Vec::new();
        let mut col_ptr: Vec<usize> = vec!(0);
        let mut idx: usize = 0;
        let mut non_zeros: usize = 0;

        for _ in 0..col {
            for j in 0..row {
                let value: T = reorder_v[idx];
                if !value.is_zero() {
                    values.push(value);
                    row_index.push(j);
                    non_zeros += 1;
                }
                idx += 1
            }
            col_ptr.push(non_zeros);
        }
        Self {
            capability, non_zeros, row, col, col_ptr, row_index, values
        }
    }
}
