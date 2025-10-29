// Suppose we have a file system that stores both files and directories. In text form, it looks like this (with ⟶ representing the tab character):
//
// dir
// ⟶ subdir1
// ⟶ ⟶ file1.ext
// ⟶ ⟶ subsubdir1
// ⟶ subdir2
// ⟶ ⟶ subsubdir2
// ⟶ ⟶ ⟶ file2.ext
//
// If we were to write this representation in code, it will look like this: "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext". Note that the '\n' and '\t' are the new-line and tab characters.
//
// Every file and directory has a unique absolute path in the file system, which is the order of directories that must be opened to reach the file/directory itself, all concatenated by '/'s. Using the above example, the absolute path to file2.ext is "dir/subdir2/subsubdir2/file2.ext". Each directory name consists of letters, digits, and/or spaces. Each file name is of the form name.extension, where name and extension consist of letters, digits, and/or spaces.

// Given a string input representing the file system in the explained format, return the length of the longest absolute path to a file in the abstracted file system. If there is no file in the system, return 0.

// Note that the testcases are generated such that the file system is valid and no file or directory name has length 0.

struct Solution;
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
      let mut max_len = 0;
      let mut level_sums: Vec<i32> = vec![0];

      for fs_entry in input.split("\n").collect::<Vec<&str>>() {
        let entry_level = fs_entry.matches("\t").count();

        match entry_level {
          // Start the calculations Vec
          0 => { level_sums[0] = fs_entry.len() as i32; }
          _ => {
            let fs_entry = fs_entry.trim_start_matches("\t");
            if level_sums.len() < entry_level + 1 {
              level_sums.push(0);
            }
            level_sums[entry_level] = level_sums[entry_level - 1] + fs_entry.len() as i32;
          }
        }

        if let Some(_) = fs_entry.rfind(".") {
          max_len = max_len.max(level_sums[entry_level] + entry_level as i32);
        }
      }
      max_len
    }
}