using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectoryList : List<SliceDirectory>
    {
        public SliceDirectoryList(string root)
        {
            // Get the major part of the assembly version
            var fvi = FileVersionInfo.GetVersionInfo(Assembly.GetExecutingAssembly().Location);
            var currentMajor = fvi.FileMajorPart;

            // Only add slices from the directories with same major version
            foreach (var dir in new DirectoryInfo(root).EnumerateDirectories())
            {
                var semVerName = SemVerNameParser.Parse(dir.Name);
                if (semVerName?.Major == currentMajor)
                    Add(new SliceDirectory(dir.FullName, semVerName));
            }
        }        
    }
}