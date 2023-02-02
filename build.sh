for dir in battery weather wifi;
  do cd "$dir"; cargo update; cargo build --release; strip target/release/$dir; cd ..
done

