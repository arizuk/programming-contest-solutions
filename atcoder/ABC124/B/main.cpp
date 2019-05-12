#include <iostream>
using namespace std;
typedef long long ll;

#define REP(i, n) for(int i = 0; i < (n); i++)
#define REP1(i, n) for(int i = 1; i <= (n); i++)

int main() {
  ll N;
  cin >> N;

  ll H[N];
  REP(i, N) cin >> H[i];

  ll ans = 0;
  REP(h, N) {
    REP(i, h) {
      if (H[i] > H[h]) {
        ans -= 1;
        break;
      }
    }
    ans += 1;
  }
  cout << ans << endl;
}