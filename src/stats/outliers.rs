use std::mem;

pub fn outlier_bound(data: &mut Vec<f64>, factor: f64) -> (f64, f64) {
    // sort descendingly
    reverse_sort(data);

    let mc = f64::min(medcouple(&data), 0.6);
    let q1 = data[(data.len() - 1) * 3 / 4];
    let q3 = data[(data.len() - 1) / 4];
    let iqr = q3 - q1;

    if mc > 0.0 {
        (
            q1 - factor * f64::exp(-4.0 * mc) * iqr,
            q3 + factor * f64::exp(3.0 * mc) * iqr,
        )
    } else {
        (
            q1 - factor * f64::exp(-3.0 * mc) * iqr,
            q3 + factor * f64::exp(4.0 * mc) * iqr,
        )
    }
}

// argument must be sorted in reverse order
fn medcouple(x: &[f64]) -> f64 {
    let eps1 = core::f64::EPSILON;
    let eps2 = core::f64::MIN_POSITIVE;

    let n = x.len();
    let n2 = (n - 1) / 2;

    if n < 3 {
        return 0.0;
    }

    let mut z = x.to_vec();
    // reverse_sort(&mut z);

    let mut zmed = if n % 2 == 1 {
        z[n2]
    } else {
        (z[n2] + z[n2 + 1]) * 0.5
    };

    if (z[0] - zmed).abs() < eps1 * (eps1 + zmed.abs()) {
        return -1.0;
    } else if (z[n - 1] - zmed).abs() < eps1 * (eps1 + zmed.abs()) {
        return 1.0;
    }

    z.iter_mut().for_each(|v| *v -= zmed);

    let zden = 0.5 / f64::max(z[0], -z[n - 1]);
    z.iter_mut().for_each(|v| *v *= zden);
    zmed *= zden;

    let zeps = eps1 * (eps1 + zmed.abs());

    let mut zplus = Vec::with_capacity(n / 2 + 1);
    let mut zminus = Vec::with_capacity(n / 2 + 1);
    for zi in &z {
        if *zi >= -zeps {
            zplus.push(*zi);
        }

        if zeps >= *zi {
            zminus.push(*zi);
        }
    }

    let (n_plus, n_minus) = (zplus.len(), zminus.len());

    let h_kern = |i: usize, j: usize| {
        let a = zplus[i];
        let b = zminus[j];

        if (a - b).abs() <= 2.0 * eps2 {
            let t1 = n_plus;
            let t2 = i + j + 1;
            if t1 > t2 {
                1.0
            } else if t1 < t2 {
                -1.0
            } else {
                0.0
            }
        } else {
            (a + b) / (a - b)
        }
    };

    let mut l = vec![0isize; n_plus];
    let mut r = vec![n_minus as isize - 1; n_plus];

    let mut ltot = 0;
    let mut rtot = n_minus * n_plus;
    let medc_idx = rtot / 2;

    let mut aw = Vec::<(f64, usize)>::with_capacity(n_plus);
    let mut p = vec![0isize; n_plus];
    let mut q = vec![0isize; n_plus];

    while rtot - ltot > n_plus {
        aw.clear();

        for (i, (li, ri)) in l.iter().zip(r.iter()).enumerate() {
            if li <= ri {
                aw.push((h_kern(i, (li + ri) as usize / 2), (ri - li + 1) as usize));
            }
        }

        let am = wmedian(&mut aw);
        let am_eps = eps1 * (eps1 + am.abs());

        let mut j = 0isize;
        for (i, pi) in p.iter_mut().enumerate().rev() {
            while j < n_minus as isize && h_kern(i, j as usize) - am > am_eps {
                j += 1;
            }
            *pi = j - 1;
        }

        j = n_minus as isize - 1;
        for (i, qi) in q.iter_mut().enumerate() {
            while j >= 0 && h_kern(i, j as usize) - am < -am_eps {
                j -= 1;
            }
            *qi = j + 1;
        }

        let sump = p.iter().sum::<isize>() + n_plus as isize;
        let sumq = q.iter().sum::<isize>();

        if medc_idx as isize <= sump - 1 {
            mem::swap(&mut r, &mut p);
            rtot = sump as usize;
        } else if medc_idx as isize > sumq - 1 {
            mem::swap(&mut l, &mut q);
            ltot = sumq as usize;
        } else {
            return am;
        }
    }

    let mut a = z;
    a.clear();
    for (i, (li, ri)) in l.iter().zip(r.iter()).enumerate() {
        for j in *li..=*ri {
            a.push(h_kern(i, j as usize));
        }
    }

    *order_stat::kth_by(&mut a, medc_idx - ltot, |a, b| b.partial_cmp(a).unwrap())
}

fn wmedian(aw: &mut [(f64, usize)]) -> f64 {
    let n = aw.len();
    let wtot = aw.iter().map(|v| v.1).sum();

    let mut beg = 0;
    let mut end = n - 1;
    let mut k = 0;

    while end - beg > 1 {
        let mid = (beg + end) / 2;
        order_stat::kth_by(&mut aw[beg..=end], mid - beg, |a, b| {
            b.0.partial_cmp(&a.0).unwrap()
        })
        .0;

        let wleft = k + aw[beg..mid].iter().map(|v| v.1).sum::<usize>();

        if wleft + wleft > wtot {
            end = mid;
        } else {
            beg = mid;
            k = wleft;
        }
    }

    let wleft = k + aw[beg].1;
    if wleft + wleft > wtot {
        aw[beg].0
    } else if wleft + wleft == wtot {
        (aw[beg].0 + aw[beg + 1].0) * 0.5
    } else {
        aw[beg + 1].0
    }
}

fn reverse_sort(a: &mut [f64]) {
    a.sort_unstable_by(|a, b| b.partial_cmp(&a).unwrap());
}
