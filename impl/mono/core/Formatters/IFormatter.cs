using System.Text;
using sb.Core.Slices;

namespace sb.Core.Formatters
{
    public interface IFormatter
    {
        void Write(Slice slice, StringBuilder sb);
        void Write(SliceSection section, StringBuilder sb);
    }
}