static class Globals
{
    static string VERSION = "0.2.1";
    static string LICENSE = "GPLV3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.";
    static string AUTHOR = "Olivier Thornton";

    public static void help()
    {
        System.Console.WriteLine("Usage: srm [OPTION]... [FILE]...");
        System.Console.WriteLine("Securely remove files or directories.\n");
        System.Console.WriteLine("  -r, --recursive\tremove directories and their contents recursively");
        System.Console.WriteLine("  -p, --passes\t\tset the number of passes (default is 10)");
        System.Console.WriteLine("  -v, --verbose\t\texplain what is being done");
        System.Console.WriteLine("      --version\t\toutput version information and exit");
    }

    public static void version()
    {
        System.Console.WriteLine(@$"srm {VERSION}
License {LICENSE}
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
        
Written by {AUTHOR}.");
    }
}

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

        bool debug = true;

        int counter = 0;
        // loop through args
        foreach (string arg in args)
        {
            if (debug) { System.Console.WriteLine(counter + ": " + arg); }

            // if first arg set to path
            if (counter == args.Length - 1 && arg != "-r" && arg != "-p" && arg != "-v" && arg != "--help")
            {
                // set path to arg for use outside of the loop
                path = arg;
            }
            if (arg == "-r" || arg == "--recursive")
            {
                recursive = true;
            }
            {
                recursive = true;
            }
            if (arg == "-p" || arg == "--passes")
            {
                // set passes to the next arg
                passes = int.Parse(args[counter + 1]);
            }
            if (arg == "-v" || arg == "--verbose")
            {
                verbose = true;
            }
            if (arg == "--help")
            {
                Globals.help();
                return;
            }
            if (arg == "--version")
            {
                Globals.version();
                return;
            }
            counter++;
        }

        // if debug, print the args
        if (debug)
        {
            System.Console.WriteLine("\nPath: " + path);
            System.Console.WriteLine("Passes: " + passes);
            System.Console.WriteLine("Recursive: " + recursive);
            System.Console.WriteLine("Verbose: " + verbose);
        }


        if (path == "")
        {
            // print the usage
            System.Console.WriteLine("srm: missing operand\nTry 'srm --help' for more information.");
            return;
        }
        if (path == "/" || path.ToLower() == "c" || path.ToLower() == "c:")
        {
            System.Console.WriteLine("Cannot delete root directory");
            return;
        }

        if (path == "*" || path == "./*" || path == ".\\*")
        {
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
            System.Console.WriteLine($"srm: cannot remove '{path}': Is a directory");
        }
        else if (Directory.Exists(path) && recursive)
        {
            if (debug) { System.Console.WriteLine("Path is a directory. Calling delete folder"); }
            // if path is a directory and '-r' is in args, delete the directory
            deleteFolder(path, passes, verbose, debug);
            // delete folder at path
            if (verbose) { System.Console.Write($"Deleting directory '{path}' . . .\t\t"); }
            Directory.Delete(path);
            if (verbose) { System.Console.WriteLine("Done"); }
        }
        else
        {
            if (File.Exists(path)) { deleteFile(path, passes, verbose, debug); }
            else { System.Console.WriteLine($"rm: cannot remove '{path}': No such file or directory"); }
        }
        if (debug) { System.Console.WriteLine("Done"); }
    }
    static void deleteFolder(string path, int passes, bool verbose, bool debug)
    {
        // if path ends with . or .., skip it
        if (path.EndsWith(".") || path.EndsWith("..")) { return; }
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
                if (verbose) { System.Console.Write($"Deleting directory '{path}' . . .\t\t"); }
                Directory.Delete(file);
                if (verbose) { System.Console.WriteLine("Done"); }

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
            if (verbose) { System.Console.Write($"Deleting '{path}' . . .\t\t"); }
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
