OUTPUT_FOLDER="experiments/exp1"

mkdir -p $OUTPUT_FOLDER

for I in {1..10}
do
  cargo run | tee "$OUTPUT_FOLDER/result$I.txt"
done
