using System.Collections.Generic;
using System.IO;
using cb.Utils;

namespace cb.Slices
{
    public class SliceDirectory
    {
        private readonly string _root;

        public SliceDirectory(string root)
        {
            _root = root;
        }

        public IList<Slice> FindByOs(string osName)
        {
            var list = new List<Slice>();
            ScanFiles(_root, list, osName);
            foreach (var dir in Directory.EnumerateDirectories(_root, "*.*", SearchOption.AllDirectories))
            {
                ScanFiles(dir, list, osName);
            }            
            return list;
        }

        private static void ScanFiles(string dir, IList<Slice> list, string osName)
        {
            foreach (var path in Directory.EnumerateFiles(dir))
            {
                var fileName = Path.GetFileName(path);
                if (fileName == null || fileName.StartsWith("."))
                    continue;

                var ext = Path.GetExtension(path);
                if (ext == ".md" || ext == ".txt")
                    continue;

                var fi = new FileInfo(path);
                var nameParts = fi.Name.Split("_"); // handles names like debian-8.2_jekyll-3.0 and possibly with more underscores
                fileName = nameParts.Length == 1 ? nameParts[0] : nameParts[nameParts.Length - 1]; 
                var semName = new SemName(fileName);

                var lines = File.ReadAllLines(path);
                var slice = new Slice(semName, lines);

                if (slice.OsList.Contains(osName))
                    list.Add(slice);
            }
        }
    }
}