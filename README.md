# The Rolling Stats

Rolling Stats is a high-performance RESTful service designed to ingest batches of data points and
return calculated statistics. The service aggregates data points by symbol and calculates statistics
for the last 10^k values, where k ranges from 1 to 8.

This project leverages Elixir and Phoenix for the web server, combined with Rust for
high-performance calculations. The integration between these technologies is achieved using Rustler.

## Getting Started

### Running with Docker

1. Build the Docker image:

```bash
docker build -t rolling_stats .
```

2. Run the container:

```bash
docker run -p 4000:4000 rolling_stats
```

The API will be accessible at `http://localhost:4000`.

### Local Development

1. Install Elixir and Rust on your system.
2. Install dependencies:

```bash
mix deps.get
```

3. Compile the project:

```bash
mix compile
```

4. Start the Phoenix server:

```bash
mix phx.server
```

The API will be accessible at `http://localhost:4000`.

## API Endpoints

### Add Batch

Adds a batch of data points for a specific symbol.

- **URL**: `/add_batch`
- **Method**: POST
- **Content-Type**: application/json

**Request Payload**:

```json
{
  "values": [1.0, 2.0, 3.0],
  "symbol": "a"
}
```

**Response**: 201 Created

### Get Stats

Retrieves statistics for a specific symbol and k value.

- **URL**: `/stats`
- **Method**: GET
- **Query Parameters**:
    - `symbol`: The symbol to retrieve stats for
    - `k`: The exponent for the number of data points (1-8)

**Example**: `GET /stats?symbol=a&k=3`

**Response Payload**:

```json
{
  "max": 3.0,
  "min": 1.0,
  "last": 3.0,
  "average": 2.0,
  "variance": 0.6666666666666666
}
```

**Response**: 200 OK

## Implementation Details

Rolling Stats is built using a combination of technologies:

- [**Elixir**](https://elixir-lang.org) and [**Phoenix**](https://www.phoenixframework.org): Provide
  a scalable and developer-friendly web application framework.
- [**Rust**](https://www.rust-lang.org): Handles performance-critical calculations.
- [**Rustler**](https://github.com/rusterlium/rustler): Enables seamless integration between Elixir
  and Rust through NIFs (Native Implemented
  Functions).

### Key Components

- **Phoenix**: Handles HTTP requests and routing.
- **RollingStats.Native**: Elixir module interfacing with Rust logic.
- **lib.rs**: Entry point for Rust logic, defining `add_batch` and `get_stats` functions.
- **DashMap**: Concurrent hashmap for non-blocking access to data stored per symbol.
- **MultiRollingStats**: Manages multiple `RollingStats` instances for different exponents.
- **RollingStats**: Efficiently maintains rolling statistics using a `VecDeque` and `BTreeMap`.

### Implementation

Phoenix handles HTTP requests, routing them to the appropriate handlers. The `RollingStats.Native`
module in Elixir serves as an interface to the Rust logic. In Rust, `lib.rs` acts as the entry point
for the core logic, defining two key functions: `add_batch` and `get_stats`. These functions operate
on a static `ALL_STATS` structure.

`ALL_STATS` is implemented using `DashMap`, a concurrent hashmap that enables non-blocking access to
data stored under separate symbols, ensuring thread-safety and high performance. Each value
in `DashMap` is of type `MultiRollingStats`, which maintains a hashmap of `RollingStats` instances
for each exponent (corresponding to the appropriate number of data points).

The `RollingStats` struct is responsible for statistical calculations. It contains a `VecDeque` of
the last 10^k data points and necessary data for efficient statistics
calculation: `max_size`, `sum`, `sum_squares` and `index`. These values are updated on each batch
addition. The `index` is implemented using a `BTreeMap`, which maintains ordered values and allows
for O(1) time complexity when fetching min and max values, while new value insertions are performed
in O(log n) time.

Values of `sum` and `sum_squares` are calculated based on added and removed values from the queue on
each batch. This approach, which avoids iterating over the entire queue, enables the calculation of
rolling variance and average in O(1) time.

### Complexity

- **Adding a data point**: O(log n) time complexity (due to updating the `BTreeMap` index)
- **Retrieving statistics**: O(1) time complexity (thanks to pre-calculated values)

## Testing

### Load Tests

Load tests are performed using [Vegeta](https://github.com/tsenart/vegeta). To run the load tests:

1. Navigate to the `load_test` directory.
2. Execute the load test script:

```bash
./load_test.sh
```

This script simulates 300 requests per second for 5 minutes, evenly split between adding batches and
retrieving stats. Results are visualized in a latency plot saved
as [plot.html](load_test%2Fplot.html).
It demonstrates that despite sending a total of 450M data points, the latency remained constant,
empirically suggesting a low time complexity.

### Unit Tests

Run the unit test suite with:

```bash
mix test
```
