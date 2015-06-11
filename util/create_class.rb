#!/usr/bin/env ruby

# create_class.rb [OPTIONS] NAMESPACE CLASS
#  - don't create a object source
#  - final
#  - struct
#  - rule of 0 (implicit / explicit)
#  - rule of 5
#  - documentation

require 'optparse'

# 
# parse options
#

$create_src = true
$doc = true
$final = false
$explicit = false
$five = false
$struct = false

opt_banner = 'Usage: create_class.rb [OPTIONS] NAMESPACE CLASS'
options = {}
OptionParser.new do |opts|
  opts.banner = opt_banner

  opts.separator ''
  opts.separator 'Options:'

  opts.on('-S', '--no-source', "Don't create a source file")      { |s| $create_src = s }
  opts.on('-D', '--no-doc', "Don't generate documentation")       { |d| $doc = d        }
  opts.on('-f', '--final', "Class is final")                      { |f| $final = f      }
  opts.on('-x', '--explicit', "Explicit default constructors")    { |x| $explicit = x   }
  opts.on('-5', '--five', "Create 'rule-of-five' constructors")   { |i| $five = i       }
  opts.on('-s', '--struct', "Struct instead of classes")          { |s| $struct = s     }

end.parse!

if ARGV.length != 2
  STDERR.puts opt_banner
  exit 1
end
$namespace = ARGV[0]
$class = ARGV[1]

# 
# generate header
#
header = <<-eos
// Copyright 2015 André Wagner

#ifndef %uns%_%uclass%_H_
#define %uns%_%uclass%_H_

using namespace std;

namespace %ns% {

%classkw% %class% %final%{
public:
    %class%(); %destructor% %five% %explicit%
};
%doc%
}  // namespace %ns%

#endif  // %uns%_%uclass%_H_

// vim: ts=4:sw=4:sts=4:expandtab
eos

header.gsub! '%uns%', $namespace.upcase
header.gsub! '%uclass%', $class.upcase
header.gsub! '%classkw%', $struct ? 'struct' : 'class'
header.gsub! '%ns%', $namespace
header.gsub! '%class%', $class
header.gsub! '%final%', $final ? 'final ' : ''
if $explicit
  header.gsub! '%destructor%', "\n    " + ($final ? '' : 'virtual ') + "~#{$class}();"
  header.gsub! '%explicit%', "\nprivate:
    #{$class}(#{$class} const&) = delete;
    #{$class}(#{$class}&&) = delete;
    #{$class}& operator=(#{$class} const&) = delete;
    #{$class}& operator=(#{$class}&&) = delete;\n"
else
  header.gsub! '%explicit%', ''
end
if $five
  header.gsub! '%destructor%', "\n    " + ($final ? '' : 'virtual ') + "~#{$class}();\n"
  header.gsub! '%five%', "
    #{$class}(#{$class} const& o);
    #{$class}(#{$class}&& o);
    #{$class}& operator=(#{$class} const& o);
    #{$class}& operator=(#{$class}&& o);"
else
  header.gsub! '%five%', ''
end
header.gsub! '%destructor%', ''
if $doc
  header.gsub! '%doc%', "\n/*@\nclass #{$class} {\n    +#{$class}()\n}\n@*/\n"
else
  header.gsub! '%doc%', ''
end

# 
# generate source
# 
src = ''
if $create_src
  src = <<-eos
// Copyright 2015 André Wagner

#include "%dclass%.h"


namespace %ns% {


%class%::%class%()
{
}


eos

  if $five
    src += <<-eos
%class%::%class%(%class% const& o)
{
    (void) o;
}


%class%::%class%(%class%&& o)
{
    (void) o;
}


%class%::operator=(%class% const& o)
{
    (void) o;
}


%class%::operator=(%class%&& o)
{
    (void) o;
}


eos
  end

  src += <<-eos
}  // namespace %ns%

// vim: ts=4:sw=4:sts=4:expandtab
eos
end

src.gsub! '%dclass%', $class.downcase
src.gsub! '%ns%', $namespace
src.gsub! '%class%', $class


# 
# create header and source
#
File.open("#{$namespace.downcase}/#{$class.downcase}.h", 'w') { |f| f.write(header) }
File.open("#{$namespace.downcase}/#{$class.downcase}.cc", 'w') { |f| f.write(src) } if $create_src

# vim: ts=2:sw=2:sts=2:expandtab
