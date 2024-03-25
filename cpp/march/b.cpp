#include <algorithm> // min, max, swap, sort, reverse, lower_bound, upper_bound
#include <bitset>    // bitset
#include <cctype>    // isupper, islower, isdigit, toupper, tolower
#include <cstdint>   // int64_t, int*_t
#include <cstdio>    // printf
#include <deque>     // deque
#include <iostream>  // cout, endl, cin
#include <map>       // map
#include <queue>     // queue, priority_queue
#include <set>       // set
#include <stack>     // stack
#include <string>    // string, to_string, stoi
#include <tuple>     // tuple, make_tuple
#include <unordered_map> // unordered_map
#include <unordered_set> // unordered_set
#include <utility>       // pair, make_pair
#include <vector>        // vector

using namespace std;
typedef long long ll;

using P = pair<int, int>;

int main() {
  int N;
  cin >> N;

  vector<ll> A(N), S(N - 1), T(N - 1);

  for (int i = 0; i < N; i++) {
    cin >> A[i];
  }

  for (int i = 0; i < N - 1; i++) {
    cin >> S[i] >> T[i];
  }

  for (int i = 0; i < N - 1; i++) {
    A[i + 1] += (A[i] / S[i]) * T[i];
  }

  cout << A[N - 1] << endl;

  return 0;
}
