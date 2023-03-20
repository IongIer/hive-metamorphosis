# hive-metamorphosis

Small scripts that transform different ways of representing games of Hive from one format to another.

Currently only a PGN -> [UHP string](https://github.com/jonthysell/Mzinga/wiki/UniversalHiveProtocol) converter is implemented

Usage: "hive-metamorphosis -p /path/to/pgn_file_or_folder -m uhp" will convert a single pgn/all the pgn's in the provided folder into uhp strings in a /uhp folder under the provided path, the optional -v flag also outputs the converted strings to the terminal
