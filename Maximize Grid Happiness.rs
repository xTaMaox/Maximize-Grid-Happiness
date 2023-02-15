impl Solution {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let mut dp = vec![vec![vec![vec![vec![0; 64]; 64]; 7]; 7]; 25];
        Self::dfs(m, n, 0, introverts_count, extroverts_count, 0, 0, &mut dp)
    }

    fn n_cost(_m: i32, n: i32, i: i32, j: i32, mask_in: i32, mask_ex: i32, d: i32) -> i32 {
        let mut diff = 0;
        let up = 1 << (n - 1);
        if j > 0 && (mask_in & 1) > 0 {
            diff += d - 30;
        }
        if i > 0 && (mask_in & up) > 0 {
            diff += d - 30;
        }
        if j > 0 && (mask_ex & 1) > 0 {
            diff += d + 20;
        }
        if i > 0 && (mask_ex & up) > 0 {
            diff += d + 20;
        }
        diff
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        m: i32,
        n: i32,
        p: i32,
        in_: i32,
        ex: i32,
        mask_in: i32,
        mask_ex: i32,
        dp: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>,
    ) -> i32 {
        let i = p / n;
        let j = p % n;
        if i >= m {
            return 0;
        }
        if dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] > 0 {
            return dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] - 1;
        }
        let n_mask_in = (mask_in << 1) & 63;
        let n_mask_ex = (mask_ex << 1) & 63;
        let mut res = Self::dfs(m, n, p + 1, in_, ex, n_mask_in, n_mask_ex, dp);
        if in_ > 0 {
            let diff = 120 + Self::n_cost(m, n, i, j, mask_in, mask_ex, -30);
            res = res.max(diff + Self::dfs(m, n, p + 1, in_ - 1, ex, n_mask_in + 1, n_mask_ex, dp));
        }
        if ex > 0 {
            let diff = 40 + Self::n_cost(m, n, i, j, mask_in, mask_ex, 20);
            res = res.max(diff + Self::dfs(m, n, p + 1, in_, ex - 1, n_mask_in, n_mask_ex + 1, dp));
        }
        dp[p as usize][in_ as usize][ex as usize][mask_in as usize][mask_ex as usize] = res + 1;
        res
    }
}