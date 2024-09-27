use pyo3::prelude::*;
use rand::seq::SliceRandom;

/// Function to generate a random joke
#[pyfunction]
fn get_random_joke() -> String {
    let jokes = vec![
        "Why don’t skeletons fight each other? They don’t have the guts.",
        "What do you call cheese that isn't yours? Nacho cheese.",
        "Why couldn’t the bicycle stand up by itself? It was two tired.",
    ];

    let mut rng = rand::thread_rng();
    let joke = jokes.choose(&mut rng).unwrap_or(&"No jokes available.");
    joke.to_string()
}

/// Create a Python module named `jokes`
#[pymodule]
fn jokes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_random_joke, m)?)?;
    Ok(())
}
