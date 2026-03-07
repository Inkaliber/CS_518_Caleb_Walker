(ns stable-dedup.core)

;; ---------- Policy abstraction ----------

(defprotocol SeenPolicy
  (mark-if-new [policy x]
    "Returns [updated-policy is-new?]."))

;; ---------- Policy implementations ----------

(defrecord HashSeenPolicy [seen]
  SeenPolicy
  (mark-if-new [policy x]
    (if (contains? seen x)
      [policy false]
      [(->HashSeenPolicy (conj seen x)) true])))

(defrecord LinearSeenPolicy [seen]
  SeenPolicy
  (mark-if-new [policy x]
    (if (some #(= % x) seen)
      [policy false]
      [(->LinearSeenPolicy (conj seen x)) true])))

(defn make-hash-seen-policy []
  (->HashSeenPolicy #{}))

(defn make-linear-seen-policy []
  (->LinearSeenPolicy []))

;; ---------- Core algorithm ----------

(defn stable-dedup-with
  [policy xs]
  (first
    (reduce
      (fn [[result current-policy] x]
        (let [[next-policy is-new?] (mark-if-new current-policy x)]
          [(if is-new? (conj result x) result)
           next-policy]))
      [[] policy]
      xs)))

;; ---------- Public APIs ----------

(defn stable-dedup
  [xs]
  (stable-dedup-with (make-hash-seen-policy) xs))

(defn stable-dedup-no-hash
  [xs]
  (stable-dedup-with (make-linear-seen-policy) xs))

;; ---------- Example usage ----------

(defn -main []
  (let [ints [3 1 3 2 1 4 2]
        words ["cat" "dog" "cat" "bird" "dog"]
        no-hash-list [3 1 3 2 1 4 2]]

    (println "Original ints:      " ints)
    (println "Deduped ints:       " (stable-dedup ints))
    (println)

    (println "Original strings:   " words)
    (println "Deduped strings:    " (stable-dedup words))
    (println)

    (println "Original no-hash:   " no-hash-list)
    (println "Deduped no-hash:    " (stable-dedup-no-hash no-hash-list))))