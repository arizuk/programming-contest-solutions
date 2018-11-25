// int64_t binsearch(int64_t l, int64_t r, UnaryPredicate p) {
//     assert (l <= r);
//     -- l;
//     while (r - l > 1) {
//         int64_t m = l + (r - l) / 2;  // avoid overflow
//         (p(m) ? r : l) = m;
//     }
//     return r;
// }
