using System.Collections.Generic;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceList 
    {
        private readonly List<Slice> _slices = new List<Slice>();

        public IList<SemVerInfo> MissingVersions { get; } = new List<SemVerInfo>();

        public Slice this[int index] => _slices[index];

        public void Add(Slice slice)
        {
            _slices.Add(slice);
        }

        public Slice FindSlice(SemVerInfo svi)
        {
            var slice = _slices.Find(item => item.SemVerInfo.Name == svi.Name && item.SemVerInfo.CompareByNameSemVer(svi) >= 0);
            if (slice == null)
            {
                if (!MissingVersions.Contains(svi))
                    MissingVersions.Add(svi);
                slice = new MissingSlice(svi);
            }
            return slice;
        }

        public IList<Slice> FindSlices(SemVerInfo[] svis)
        {
            var slices = new List<Slice>();
            foreach (var svi in svis)
            {
                var slice = FindSlice(svi);
                if (!slices.Contains(slice))
                {
                    slices.Add(slice);
                }
            }
            return slices;
        }

        public void Sort()
        {
            _slices.Sort((x, y) => y.SemVerInfo.CompareTo(x.SemVerInfo));
        }
    }
}