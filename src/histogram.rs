// plan
// first implement Numeric ones
// for numeric ones, it can adjust x axis intervals (merge or split bins)
// for numeric ones, it can implement custom formatters (20210304 -> "2021/03/04")
// for numeric ones, it can choose to sort or not
// for string ones, it first give numeric ids to identifiers, run a pipeline for numeric ones,
// then map the ids back to identifiers using a formatter
pub struct Histogram {
    data: HistogramData,
    width: usize,
    height: usize,
    y_label_formatter: Arc<dyn NumberFormatter>,
    merge_x_labels: MergeX,

    // it only works for numeric x labels
    x_label_formatter: Arc<dyn NumberFormatter>,
}

impl Histogram {
    pub fn set_numeric_data<T: TryInto<Ratio>>(&mut self, ns: &[T]) -> &mut Self {}

    pub fn set_figure_data<T: ToString>(&mut self, ns: &[T]) -> &mut Self {}
}
