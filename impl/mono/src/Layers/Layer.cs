using System.Collections.Generic;
using System.Linq;
using System.Text;
using sb.Slices;
using sb.Utils;

namespace sb.Layers
{
    public class Layer
    {
        private readonly Slice _slice;

        public Layer(Slice slice)
        {
            _slice = slice;
        }

        public SemVerName SemVerName => _slice.SemVerName;
        public IList<SliceSection> Sections => _slice.Sections;
        public LayerList Dependencies { get; } = new LayerList();
        public bool Written { get; set; }

        public void FindDependenciesRecursive(LayerList registryLayers)
        {
            foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.DEP))
            {
                foreach (var line in section.Lines)
                {
                    var svn = SemVerNameParser.Parse(line);
                    var dep = registryLayers.FindLayer(svn.Name);
                    if (dep != null && !Dependencies.Contains(dep))
                    {
                        Dependencies.Add(dep);
                        dep.FindDependenciesRecursive(registryLayers);
                    }
                }
            }

            if (Dependencies.Count == 0)
            {
                foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.OS))
                {
                    foreach (var line in section.Lines)
                    {
                        var svn = SemVerNameParser.Parse(line);
                        if (svn.Name != SemVerName.Name)
                        {
                            var os = registryLayers.FindLayer(svn.Name);
                            if (os != null && !Dependencies.Contains(os))
                            {
                                Dependencies.Add(os);
                            }
                        }
                    }
                }
            }
        }

        public void Write(StringBuilder sb)
        {
            sb.AppendLine();
            foreach (var section in _slice.Sections.Where(s => s.SectionType != SliceSection.Type.OS && s.SectionType != SliceSection.Type.DEP))
            {
                sb.AppendLine();
                sb.AppendLine(section.SectionType.ToString());
                sb.AppendLine($"# {SemVerName}");
                foreach (var line in section.Lines)
                {
                    sb.AppendLine(line);
                }
            }
            sb.AppendLine();
        }

        public override bool Equals(object obj)
        {
            var other = (Layer) obj;
            return SemVerName.Equals(other.SemVerName);
        }

        public override int GetHashCode()
        {
            return SemVerName.GetHashCode();
        }

        public override string ToString()
        {
            return _slice.SemVerName.ToString();
        }
    }
}