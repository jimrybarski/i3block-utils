for dir in battery weather wifi;
  do cd "$dir"; cargo build --release; strip target/release/$dir; cd ..
done

