## Usage

    USAGE:
        npy-write [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -d, --dtype <type>     data type, one of: u32, i32, u64, i64, f32, f64 (default: f32)
        -o, --output <FILE>    output file name (default: data.npy)
        -s, --separator <S>    character used to separate fields (default: space)

For example in

    cat lots-of-big-integers.txt | npy-write -o nicematrix.npy -d u64 -s ' '

`lots-of-big-integers.txt` might look like

    0 53 49
    0 2721 4
    0 1679 5
    0 1974 8
    3 12207 1
    1 2698 3
    1 5561 2
    3 217 2
    0 12532 2
    0 1456 2

and the output `nicematrix.npy` would then be

    In [2]: np.load('nicematrix.npy')
    Out[2]:
    array([[    0,    53,    49],
           [    0,  2721,     4],
           [    0,  1679,     5],
           [    0,  1974,     8],
           [    3, 12207,     1],
           [    1,  2698,     3],
           [    1,  5561,     2],
           [    3,   217,     2],
           [    0, 12532,     2],
           [    0,  1456,     2]], dtype=uint64)
