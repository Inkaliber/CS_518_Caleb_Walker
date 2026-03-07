#ifndef DEDUP_H
#define DEDUP_H

#include <vector>
#include <unordered_set>
#include <algorithm>
#include <type_traits>
#include <functional>
#include <utility>

// ---------- Type trait helpers ----------

template <typename, typename = void>
struct is_hashable : std::false_type {};

template <typename T>
struct is_hashable <
    T,
    std::void_t<decltype(std::hash<T>{}(std::declval<const T&>())) >
> : std::true_type {
};

template <typename, typename = void>
struct is_equality_comparable : std::false_type {};

template <typename T>
struct is_equality_comparable<
    T,
    std::void_t<decltype(std::declval<const T&>() == std::declval<const T&>())>
> : std::true_type {
};

// ---------- Policy implementations ----------

template <typename T>
class HashSeenPolicy {
public:
    explicit HashSeenPolicy(std::size_t expected_size = 0) {
        seen_.reserve(expected_size);
    }

    bool mark_if_new(const T& value) {
        return seen_.insert(value).second;
    }

private:
    std::unordered_set<T> seen_;
};

template <typename T>
class LinearSeenPolicy {
public:
    explicit LinearSeenPolicy(std::size_t expected_size = 0) {
        seen_.reserve(expected_size);
    }

    bool mark_if_new(const T& value) {
        if (std::find(seen_.begin(), seen_.end(), value) != seen_.end()) {
            return false;
        }
        seen_.push_back(value);
        return true;
    }

private:
    std::vector<T> seen_;
};

// ---------- Core algorithm ----------

template <typename T, typename Policy>
std::vector<T> stable_dedup_with(const std::vector<T>& xs, Policy policy) {
    static_assert(
        is_equality_comparable<T>::value,
        "T must support equality comparison (operator==)."
        );

    std::vector<T> result;
    result.reserve(xs.size());

    for (const auto& x : xs) {
        if (policy.mark_if_new(x)) {
            result.push_back(x);
        }
    }

    return result;
}

// ---------- Public API ----------

template <typename T,
    typename std::enable_if<is_hashable<T>::value, int>::type = 0>
std::vector<T> stable_dedup(const std::vector<T>& xs) {
    return stable_dedup_with(xs, HashSeenPolicy<T>(xs.size()));
}

template <typename T,
    typename std::enable_if<!is_hashable<T>::value, int>::type = 0>
std::vector<T> stable_dedup(const std::vector<T>& xs) {
    return stable_dedup_with(xs, LinearSeenPolicy<T>(xs.size()));
}

#endif