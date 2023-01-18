class srm {
    static void Main(string[] args) {
        // get the command line input
        if (args.Length == 0) {
            // print the usage
            System.Console.WriteLine("Usage: srm <file> [passes: optional]");
            return;
        }
        string path = args[0];
        // if args[1] is null
        int passes = 5000;
        if (args.Length == 2) {
            passes = int.Parse(args[1]);
        } 

        if (path == "*") {
            // get all files in the working directory
            string[] files = Directory.GetFiles(Directory.GetCurrentDirectory());
            // loop through the files
            foreach (string file in files) {
                // delete the file
                deleteFile(file, passes);
            }
        } else {
            // delete the file
            deleteFile(path, passes);
        }
    }

    static void deleteFile(string path, int passes) {
        // open the file for reading and writing, overwrite the file 
        // with random data passes times, and then delete the file
        using (var fs = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None)) {
            // print the file name
            System.Console.Write("Deleting " + path + ". . . ");
            for (int i = 0; i < passes; i++) {
                // get the length of the file
                long length = fs.Length;
                // create a random number generator
                Random random = new Random();
                // create a byte array
                byte[] bytes = new byte[length];
                // fill the byte array with random data
                random.NextBytes(bytes);
                // set the position of the file to the beginning
                fs.Position = 0;
                // overwrite the file with random data
                fs.Write(bytes, 0, bytes.Length);
                // flush the data to the file
                fs.Flush();
            }
            // close the file
            fs.Close();
            // delete the file
            File.Delete(path);
            // print the status
            System.Console.WriteLine("Done");
        }
    }
}
