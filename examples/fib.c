
int fibonacci(int n, int b) {

  if (n == 0) {
    return 0;
  }

  if (n == 1) {
    return 1;
  }

  return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
  int n = 3;
  int f = n + 1;
  int result = fibonacci(n, f);

  return 0;
}