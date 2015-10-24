using System.IO;
using System.Linq;

namespace sb.TestBB.Utils
{
    public static class FsUtils
    {
        public static string ReplaceFileName(this string path, string newName)
        {
            return Path.Combine(Path.GetDirectoryName(path), newName);
        }

        public static void DeleteFolder(string path)
        {
            if (Directory.Exists(path))
                Directory.Delete(path, true);
        }

        public static void DeleteFolderExcept(string path, string preservedPath)
        {
            foreach (var dir in Directory.GetDirectories(path).ToList())
            {
                if (dir.ToLower() == preservedPath.ToLower())
                    continue;
                Directory.Delete(dir, true);
            }
        }

        public static bool FolderExists(string path, string mask)
        {
            foreach (var dir in Directory.EnumerateDirectories(path))
            {
                if (new DirectoryInfo(dir).Name.Contains(mask))
                    return true;
            }
            return false;
        }

        public static bool FileExists(string path, string mask)
        {
            foreach (var file in Directory.EnumerateFiles(path))
            {
                if (new FileInfo(file).Name.Contains(mask))
                    return true;
            }
            return false;
        }
    }
}