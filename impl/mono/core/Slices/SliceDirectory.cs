using System.Collections.Generic;
using System.IO;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectory
    {
        public SliceDirectory(DirectoryInfo rootDir, DirectoryInfo bunchDir, SemVerInfo bunchSvi)
        {
            RootDir = rootDir;
            BunchDir = bunchDir;
            BunchSvi = bunchSvi;
        }

        public DirectoryInfo RootDir { get; }
        public DirectoryInfo BunchDir { get; }
        public SemVerInfo BunchSvi { get; }

        public IList<Slice> Scan(SemVerInfo osInfo)
        {
            var list = new List<Slice>();
            ScanFiles(BunchDir.FullName, list, osInfo);
            foreach (var dir in Directory.EnumerateDirectories(BunchDir.FullName, "*.*", SearchOption.AllDirectories))
            {
                ScanFiles(dir, list, osInfo);
            }            
            return list;
        }

        private void ScanFiles(string dir, IList<Slice> list, SemVerInfo osInfo)
        {
            foreach (var path in Directory.EnumerateFiles(dir))
            {
                var fileName = new FileInfo(path).Name;
                if (fileName.StartsWith("."))
                    continue;

                var ext = Path.GetExtension(fileName);
                if (ext == ".md" || ext == ".txt")
                    continue;

                var relPath = path.Replace(RootDir.FullName, "");
                var svi = new SemVerInfo(BunchSvi.NameSemVer, fileName);
                var lines = File.ReadAllLines(path);
                var slice = new Slice(relPath, svi, lines);

                if (slice.SupportsOs(osInfo))
                    list.Add(slice);
            }
        }
    }
}