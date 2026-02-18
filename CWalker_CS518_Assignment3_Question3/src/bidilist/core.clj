(ns bidilist.core
  (:gen-class))

;; ---------- Bidirectional-list zipper implementation ----------

(defrecord BiLoc [cur before after])

(defn empty-bilist [] nil)

(defn bilist? [x] (instance? BiLoc x))

(defn from-seq
  "Create a bidirectional list focused at the first element of xs.
   Returns nil if xs is empty."
  [xs]
  (when-let [s (seq xs)]
    (->BiLoc (first s) '() (rest s))))

(defn current [loc] (when loc (:cur loc)))

(defn before [loc] (when loc (reverse (:before loc))))
(defn after  [loc] (when loc (:after loc)))

(defn at-start? [loc] (or (nil? loc) (empty? (:before loc))))
(defn at-end?   [loc] (or (nil? loc) (empty? (:after loc))))

(defn forward
  "Move focus one step to the right. If already at end or empty, returns loc."
  [loc]
  (if (and loc (seq (:after loc)))
    (let [a (:after loc)]
      (->BiLoc (first a)
               (conj (:before loc) (:cur loc))
               (rest a)))
    loc))

(defn back
  "Move focus one step to the left. If already at start or empty, returns loc."
  [loc]
  (if (and loc (seq (:before loc)))
    (let [b (:before loc)]
      (->BiLoc (first b)
               (rest b)
               (cons (:cur loc) (:after loc))))
    loc))

(defn move
  "Move n steps: n>0 forward, n<0 back, n=0 unchanged."
  [loc n]
  (cond
    (nil? loc) nil
    (zero? n) loc
    (pos? n)  (nth (iterate forward loc) n)
    :else     (nth (iterate back loc) (- n))))

(defn to-seq
  "Convert loc back to a plain sequence in left-to-right order."
  [loc]
  (when loc
    (concat (reverse (:before loc)) (list (:cur loc)) (:after loc))))

(defn insert-before
  "Insert x immediately before the current focus. Focus remains unchanged."
  [loc x]
  (if loc
    (update loc :before conj x)
    (->BiLoc x '() '())))

(defn insert-after
  "Insert x immediately after the current focus. Focus remains unchanged."
  [loc x]
  (if loc
    (update loc :after #(cons x %))
    (->BiLoc x '() '())))

(defn replace-current
  "Replace the focused element with x."
  [loc x]
  (when loc
    (assoc loc :cur x)))

(defn delete-current
  "Delete the focused element.
   New focus is:
     - next element if it exists,
     - otherwise previous element if it exists,
     - otherwise empty (nil)."
  [loc]
  (when loc
    (cond
      (seq (:after loc))
      (->BiLoc (first (:after loc)) (:before loc) (rest (:after loc)))

      (seq (:before loc))
      (->BiLoc (first (:before loc)) (rest (:before loc)) '())

      :else
      nil)))

(defn index
  "0-based index of the focus (O(1))."
  [loc]
  (if loc (count (:before loc)) 0))

(defn count-bilist
  "Total number of elements (O(n))."
  [loc]
  (if loc (+ (count (:before loc)) 1 (count (:after loc))) 0))

;; ---------- Validation + Pretty-print for testing ----------

(defn valid-bilist?
  "Checks shape invariants for our representation."
  [loc]
  (or (nil? loc)
      (and (bilist? loc)
           ;; before/after should be seqs (lists) or empty seqs
           (sequential? (:before loc))
           (sequential? (:after loc)))))

(defn show
  "Pretty view of the current state (uses bilist?)."
  [label loc]
  (println "----" label "----")
  (println "bilist? :" (bilist? loc))
  (println "valid? :" (valid-bilist? loc))
  (println "as-seq  :" (if (bilist? loc) (vec (to-seq loc)) []))
  (println "before  :" (if (bilist? loc) (vec (before loc)) []))
  (println "current :" (current loc))
  (println "after   :" (if (bilist? loc) (vec (after loc)) []))
  (println "index   :" (index loc))
  (println "count   :" (count-bilist loc))
  (println "start?  :" (at-start? loc))
  (println "end? :" (at-end? loc))
  (println))

;; ---------- Main / test driver ----------

(defn -main
  [& _args]
  (println "Bidirectional list (zipper) test run\n")

  (let [z0 (from-seq [1 2 3 4])]
    (show "z0 = from-seq [1 2 3 4]" z0)

    (let [z1 (forward z0)]
      (show "z1 = forward z0" z1)

      (let [z2 (-> z1 (insert-before 99) (insert-after 100))]
        (show "z2 = insert-before 99, insert-after 100 (focus unchanged)" z2)

        (let [z3 (replace-current z2 42)]
          (show "z3 = replace-current with 42" z3)

          (let [z4 (delete-current z3)]
            (show "z4 = delete-current (deletes 42)" z4)

            (let [z5 (-> z4 forward forward)]
              (show "z5 = move forward twice" z5)

              (let [z6 (back z5)]
                (show "z6 = back once" z6)

                (let [z7 (move z6 -10)] ; try to go past start
                  (show "z7 = move -10 (clamped by repeated back)" z7))))))))

    ;; Edge cases:
    (let [e (empty-bilist)]
      (show "empty-bilist" e)
      (show "insert-before into empty (creates singleton)" (insert-before e :only))
      (show "delete-current singleton => empty"
            (delete-current (from-seq [:x]))))

    (println "Done.")))
