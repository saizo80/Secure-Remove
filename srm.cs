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
        string path = args[0];
        // if args[1] is null
        int passes = 5000;
        if (args.Length == 2)
        {
            passes = int.Parse(args[1]);
        } // change this ^

        if (path == "*")
        {
            // get all files in the working directory
            string[] files = Directory.GetFiles(Directory.GetCurrentDirectory());
            // loop through the files
            foreach (string file in files)
            {
                // delete the file
                deleteFile(file, passes);
            }
        }
        else if (Directory.Exists(path) && !args.Contains("-r"))
        {
            // if path is a directory and '-r' not in args, send a warning to the user
            System.Console.WriteLine(path + " is a directory.");
            //System.Console.WriteLine("If so, use the '-r' option.");
        }
        else if (Directory.Exists(path) && args.Contains("-r"))
        {
            // if path is a directory and '-r' is in args, delete the directory
            //deleteFolder(path, passes);
        }
        else
        {
            // delete the file
            deleteFile(path, passes);
        }
    }
    static void deleteFolder(string path, int passes)
    {
        // get all files in the directory
        string[] files = Directory.GetFiles(path);
        // loop through the files
        // if the file is a directory, call deleteFolder() recursively
        // else, call deleteFile()
        foreach (string file in files)
        {
            if (Directory.Exists(file))
            {
                deleteFolder(file, passes);
                // delete the directory
                Directory.Delete(file);
            }
            else
            {
                deleteFile(file, passes);
            }
        }
    }

    static void deleteFile(string path, int passes)
    {
        // open the file for reading and writing, overwrite the file 
        // with random data passes times, and then delete the file
        using (var fs = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None))
        {
            // print the file name
            System.Console.Write("Deleting " + path + ". . . ");
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
            System.Console.WriteLine("Done");
        }
    }
}
