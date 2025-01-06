

void arithmetic_operations() {
  int a = 10;
  int b = 20;
  int c;
  c = a + b; // 加法
  c = a - b; // 减法
  c = a * b; // 乘法
  c = a / b; // 除法
}

void bitwise_operations() {
  int a = 0xF0;
  int b = 0x0F;
  int c;
  c = a & b;  // 按位与
  c = a | b;  // 按位或
  c = a ^ b;  // 按位异或
  c = a << 2; // 左移
  c = b >> 2; // 右移
}

void control_flow() {
  for (int i = 0; i < 10; i++) { // 循环
    if (i % 2 == 0)              // 条件判断
      printf("%d\n", i);
  }
}

void memory_operations() {
  int array[10];
  memset(array, 0, sizeof(array)); // 内存填充
  for (int i = 0; i < 10; i++) {
    array[i] = i * i;
  }
}

void system_call_simulation() {
  printf("Hello, World!\n"); // 系统调用（例如 Linux 下的 write 系统调用）
}

int main() {
  arithmetic_operations();
  bitwise_operations();
  control_flow();
  memory_operations();
  system_call_simulation();
  return 0;
}
