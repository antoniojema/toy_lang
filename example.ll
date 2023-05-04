@a = global i32 4
@asdf = global i32 4
define void @main() {
    %a = alloca i32
    %a = i32 2
    %b = alloca i32
    %b = i32 1
    %d = alloca i32
    %d = i32 1
    %c = alloca i32
    %1 = load %a
    %2 = load %b
    %0 = mul i32 %1, %2
    %5 = i32 4
    %7 = load %b
    %8 = load %d
    %6 = add i32 %7, %8
    %4 = mul i32 %5, %6
    %c = add i32 %0, %4
}

