using System.Collections.Generic;
using System.IO;
using System.Linq;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectory
    {
        public SliceDirectory(DirectoryInfo rootDir, DirectoryInfo bunchDir, SemVerInfo bunchInfo)
        {
            RootDir = rootDir;
            BunchDir = bunchDir;
            OsDir = bunchDir.GetDirectories("_", SearchOption.TopDirectoryOnly).First();
            BunchInfo = bunchInfo;
        }

        public DirectoryInfo RootDir { get; }
        public DirectoryInfo BunchDir { get; }
        public DirectoryInfo OsDir { get; }
        public SemVerInfo BunchInfo { get; }

        public IList<Slice> Scan(SemVerInfo osInfo)
        {
            // first find suitable os slices in the underscore dir
            var list = FindOsSlices(osInfo);
            if (list.Count != 0)
            {
                // now loop over subdirectories (bunch root directory is excluded now)
                foreach (var dir in Directory.EnumerateDirectories(BunchDir.FullName, "*.*", SearchOption.AllDirectories))
                {
                    // skip already processed underscore directory with os slices
                    // and skip hidden directories
                    if (dir == OsDir.FullName || dir.StartsWith("."))
                        continue;
                    LoadSlices(new DirectoryInfo(dir), list);
                }
            }
            return list;
        }

        private IList<Slice> FindOsSlices(SemVerInfo osInfo)
        {
            var list = new List<Slice>();
            foreach (var fi in OsDir.GetFiles())
            {
                var info = new SemVerInfo(fi.Name);
                // load os slices with the higher or equal version that was requested
                if (info.Name == osInfo.Name && info.CompareByNameSemVer(osInfo) >= 0)
                {
                    var osSlice = LoadSlice(fi);
                    if (osSlice != null)
                        list.Add(osSlice);
                }
            }
            return list;
        }

        private void LoadSlices(DirectoryInfo dir, IList<Slice> list)
        {
            foreach (var fi in dir.GetFiles())
            {
                var slice = LoadSlice(fi);
                if (slice != null)
                    list.Add(slice);
            }
        }

        private Slice LoadSlice(FileInfo fi)
        {
            if (fi.Name.StartsWith("."))
                return null;

            var ext = Path.GetExtension(fi.Name);
            if (ext == ".md" || ext == ".txt")
                return null;

            var relPath = fi.FullName.Replace(RootDir.FullName + Path.DirectorySeparatorChar, "");
            var sliceInfo = new SemVerInfo(BunchInfo.NameSemVer, fi.Name);
            var lines = File.ReadAllLines(fi.FullName);
            var slice = new Slice(relPath, sliceInfo, lines);
            return slice;
        }
    }
}