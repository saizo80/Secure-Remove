class srm
{
    static void Main(string[] args)
    {
        // get the command line input
        if (args.Length == 0)
        {
            // print the usage
            System.Console.WriteLine("Usage: srm <file> [passes: optional]");
            return;
        }

        string path = "";
        int passes = 10;
        bool recursive = false;
        bool verbose = false;
        bool debug = false;

        int counter = 0;
        // loop through args
        foreach (string arg in args)
        {
            if (debug) { System.Console.WriteLine(counter + ": " + arg); }

            // if first arg set to path
            if (counter == 0 && arg != "-r" && arg != "-p" && arg != "-v")
            {
                // set path to arg for use outside of the loop
                path = arg;
            }
            if (arg == "-r")
            {
                recursive = true;
            }
            if (arg == "-p")
            {
                // set passes to the next arg
                passes = int.Parse(args[counter + 1]);
                if (debug) { System.Console.WriteLine("Passes: " + passes); }
            }
            if (arg == "-v")
            {
                verbose = true;
            }
            counter++;
        }

        if (debug) { System.Console.WriteLine("Path: " + path); }
        if (path == "")
        {
            // print the usage
            System.Console.WriteLine("Usage: srm <file> [passes: optional]");
            return;
        }

        if (path == "*")
        {
            if (debug) { System.Console.WriteLine("Path is *"); }
            // get all files in the working directory
            string[] folders = Directory.GetDirectories(Directory.GetCurrentDirectory());
            string[] files = Directory.GetFiles(Directory.GetCurrentDirectory());

            string[] all = new string[folders.Length + files.Length];
            folders.CopyTo(all, 0);
            files.CopyTo(all, folders.Length);

            // loop through the files
            foreach (string file in all)
            {
                if (debug) { System.Console.WriteLine("File: " + file); }
                if (Directory.Exists(file) && recursive)
                {
                    if (debug) { System.Console.WriteLine("Deleting directory: " + file); }
                    deleteFolder(file, passes, verbose, debug);
                }
                else if (Directory.Exists(file) && !recursive)
                {
                    // if the file is a directory, send a warning to the user
                    System.Console.WriteLine(file + " is a directory. . . Skipping");
                }
                else if (File.Exists(file))
                {
                    // delete the file
                    deleteFile(file, passes, verbose, debug);
                }
            }
        }
        else if (Directory.Exists(path) && !recursive)
        {
            // if path is a directory and '-r' not in args, send a warning to the user
            System.Console.WriteLine(path + " is a directory.");
            System.Console.WriteLine("To delete use the '-r' option.");
        }
        else if (Directory.Exists(path) && recursive)
        {
            if (debug) { System.Console.WriteLine("Path is a directory. Calling delete folder"); }
            // if path is a directory and '-r' is in args, delete the directory
            deleteFolder(path, passes, verbose, debug);
            // delete folder at path
            if (verbose) { System.Console.Write("Deleting directory " + path + ". . . "); }
            Directory.Delete(path);
            if (verbose) { System.Console.WriteLine("Done"); }
        }
        else if (File.Exists(path))
        {
            // delete the file
            deleteFile(path, passes, verbose, debug);
        }
        if (debug) { System.Console.WriteLine("Done"); }
    }
    static void deleteFolder(string path, int passes, bool verbose, bool debug)
    {
        if (debug) { System.Console.WriteLine("Deleting folder: " + path); }

        // get all files and folder in the directory
        string[] files = Directory.GetFiles(path);
        string[] folders = Directory.GetDirectories(path);
        string[] all = new string[folders.Length + files.Length];
        folders.CopyTo(all, 0);
        files.CopyTo(all, folders.Length);

        // loop through the files
        // if the file is a directory, call deleteFolder() recursively
        // else, call deleteFile()
        foreach (string file in all)
        {
            if (debug) { System.Console.WriteLine("File: " + file); }
            if (Directory.Exists(file))
            {
                if (debug) { System.Console.WriteLine("Deleting directory: " + file); }
                deleteFolder(file, passes, verbose, debug);
                System.Console.Write("Deleting directory " + file + ". . . ");
                Directory.Delete(file);
                System.Console.WriteLine("Done");

            }
            else if (File.Exists(file))
            {
                // delete the file
                deleteFile(file, passes, verbose, debug);
            }
        }
    }

    static void deleteFile(string path, int passes, bool verbose, bool debug)
    {
        if (debug) { System.Console.WriteLine("Deleting file called: " + path); }
        // open the file for reading and writing, overwrite the file 
        // with random data passes times, and then delete the file
        using (var fs = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None))
        {
            // print the file name
            if (verbose) { System.Console.Write("Deleting " + path + ". . . "); }
            for (int i = 0; i < passes; i++)
            {
                // get the length of the file
                long length = fs.Length;
                // create a byte array
                byte[] bytes = new byte[length];
                // fill the byte array with random data
                System.Security.Cryptography.RandomNumberGenerator.Create().GetBytes(bytes);
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
            if (verbose) { System.Console.WriteLine("Done"); }
        }
    }
}
