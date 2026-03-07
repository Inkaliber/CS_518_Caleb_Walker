(ns priority-queue.core)

(defrecord PriorityQueue [cmp items])

(defn make-priority-queue
  "Create an empty priority queue.
   The cmp function should behave like compare:
   - negative if a < b
   - zero if a = b
   - positive if a > b
   Higher priority means that an item is considered greater."
  [cmp]
  (->PriorityQueue cmp []))

(defn push
  "Add an item to the queue in sorted order."
  [pq item]
  (let [cmp (:cmp pq)
        items (:items pq)
        [front back] (split-with #(<= (cmp % item) 0) items)]
    (->PriorityQueue cmp (vec (concat front [item] back)))))

(defn pq-pop
  "Remove and return the item with the highest priority.
   Returns [item new-pq], or nil if the queue is empty."
  [pq]
  (let [items (:items pq)]
    (when (seq items)
      [(peek items)
       (->PriorityQueue (:cmp pq) (pop items))])))

(defn traverse
  "Print items from the highest priority to the lowest priority."
  [pq]
  (doseq [item (reverse (:items pq))]
    (println item)))

(defn -main []
  ;; Example: larger number = higher priority
  (let [pq1 (-> (make-priority-queue compare)
                (push 4)
                (push 1)
                (push 9)
                (push 3))]
    (println "Queue contents in priority order:")
    (traverse pq1)

    (let [[top pq2] (pq-pop pq1)]
      (println "Popped:" top)
      (println "After pop:")
      (traverse pq2)))

  ;; Example with maps
  (let [task-cmp (fn [a b] (compare (:priority a) (:priority b)))
        pq2 (-> (make-priority-queue task-cmp)
                (push {:name "Wash dishes" :priority 2})
                (push {:name "Finish project" :priority 10})
                (push {:name "Check email" :priority 4}))]
    (println "\nTasks in priority order:")
    (traverse pq2)

    (let [[top _] (pq-pop pq2)]
      (println "Popped task:" top))))