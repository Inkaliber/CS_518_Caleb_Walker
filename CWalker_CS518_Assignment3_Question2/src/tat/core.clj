(ns tat.core
  (:gen-class))

;; -----------------------------------------------------------------------------
;; Representation
;;   empty tree  => '()
;;   node        => (list value left right)   i.e. (value left right)
;; -----------------------------------------------------------------------------

(def empty-tat '())

(defn tat-empty?
  "True iff t is the empty TAT."
  [t]
  (or (nil? t) (empty? t)))

(defn node
  "Construct a TAT node."
  [v l r]
  (list v l r))

;; -----------------------------------------------------------------------------
;; insert
;; -----------------------------------------------------------------------------

(defn insert
  [t x]
  (if (tat-empty? t)
    (node x empty-tat empty-tat)
    (let [[v l r] t]
      (cond
        (< x v) (node v (insert l x) r)
        (> x v) (node v l (insert r x))
        :else   (node v l (insert r x)))))) ; duplicates go right

;; -----------------------------------------------------------------------------
;; member?
;; -----------------------------------------------------------------------------

(defn member?
  [t x]
  (loop [t t]
    (if (tat-empty? t)
      false
      (let [[v l r] t]
        (cond
          (< x v) (recur l)
          (> x v) (recur r)
          :else   true)))))

;; -----------------------------------------------------------------------------
;; delete (removes one occurrence)
;; -----------------------------------------------------------------------------

(defn- min-value
  [t]
  (let [[v l _] t]
    (if (tat-empty? l)
      v
      (recur l))))

(defn delete
  [t x]
  (if (tat-empty? t)
    empty-tat
    (let [[v l r] t]
      (cond
        (< x v) (node v (delete l x) r)
        (> x v) (node v l (delete r x))
        :else
        (cond
          (tat-empty? l) r
          (tat-empty? r) l
          :else
          (let [succ (min-value r)]
            (node succ l (delete r succ))))))))

;; -----------------------------------------------------------------------------
;; traversals
;; -----------------------------------------------------------------------------

(defn in-order
  [t f]
  (lazy-seq
    (if (tat-empty? t)
      '()
      (let [[v l r] t]
        (concat (in-order l f)
                (list (f v))
                (in-order r f))))))

(defn pre-order
  [t f]
  (lazy-seq
    (if (tat-empty? t)
      '()
      (let [[v l r] t]
        (concat (list (f v))
                (pre-order l f)
                (pre-order r f))))))

(defn post-order
  [t f]
  (lazy-seq
    (if (tat-empty? t)
      '()
      (let [[v l r] t]
        (concat (post-order l f)
                (post-order r f)
                (list (f v)))))))

;; -----------------------------------------------------------------------------
;; "main" demo
;; -----------------------------------------------------------------------------

(defn -main
  [& _args]
  (println "=== TAT demo ===")

  (let [t (-> empty-tat
              (insert 5)
              (insert 2)
              (insert 8)
              (insert 5))] ; duplicate
    (println "Tree (raw list form):" t)
    (println "Empty?:" (tat-empty? t))
    (println "Member 8?:" (member? t 8))
    (println "Member 99?:" (member? t 99))

    (println "In-order:"  (doall (in-order t identity)))
    (println "Pre-order:" (doall (pre-order t identity)))
    (println "Post-order:"(doall (post-order t identity)))

    (let [t2 (delete t 5)]
      (println "After (delete 5) once, raw:" t2)
      (println "In-order after delete:" (doall (in-order t2 identity)))
      (println "Pre-order after delete:" (doall (pre-order t2 identity)))
      (println "Post-order after delete:"(doall (post-order t2 identity))))

    (let [t3 (insert t 10)]
      (println "After (insert 10) once, raw:" t3)
      (println "In-order after delete:" (doall (in-order t3 identity)))
      (println "Pre-order after delete:" (doall (pre-order t3 identity)))
      (println "Post-order after delete:"(doall (post-order t3 identity))))

    (println "Apply expression (* 10) in-order:"
             (doall (in-order t #(* 10 %))))))