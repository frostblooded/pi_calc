echo "Running benchmarks..."
cargo bench | tee control
echo "Plotting benchmarks..."
cargo benchcmp control