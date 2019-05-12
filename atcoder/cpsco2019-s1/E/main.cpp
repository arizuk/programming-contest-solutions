#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
#define rep(i, n) for(int i = 0; i < (n); i++)
#define rep1(i, n) for(int i = 1; i <= (n); i++)

//debug
#define dump(x)  cerr << #x << " = " << (x) << endl;
#define debug(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")" << " " << __FILE__ << endl;


int main() {
  ll N, Q;
  cin >> N >> Q;
  ll A[N], L[Q], R[Q], X[Q];
  rep(i, N) cin >> A[i];

  int i = 0;

  set<ll> aa;
  rep(i, N) {
    if (aa.count(A[i])) {
      aa.erase(A[i]);
    } else {
      aa.insert(A[i]);
    }
  }

  rep(i, Q) {
    ll L, R, X;
    ll ans = 0;
    ll cnt = 0;
    cin >> L >> R >> X;
    auto it = aa.lower_bound(L);

    while(1) {
      if (it == aa.end()) break;
      ll value = *it;
      if (value > R) break;
      ans ^= value;
      it = aa.erase(it);
      cnt += 1;
    }

    if (cnt%2 == 1) {
      if (aa.count(X)) {
        aa.erase(X);
      } else {
        aa.insert(X);
      }
    }
    cout << ans << endl;
  }
}