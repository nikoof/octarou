packages.paper = with pkgs;
  stdenvNoCC.mkDerivation rec {
    name = "ocatrou-paper";
    src = ./doc/paper/src;
    buildInputs = [
      coreutils
      biber
      python311Packages.pygments
      which
      (texlive.combine {
        inherit (texlive) scheme-medium biblatex csquotes minted;
      })
    ];
    phases = ["unpackPhase" "buildPhase" "installPhase"];

    buildPhase = ''
      export PATH="${lib.makeBinPath buildInputs}"
      mkdir -p .cache/texmf-var
      which pygmentize 1> /dev/null
      env TEXMFHOME=.cache TEXMFVAR=.cache/texmf-var \
        latexmk -interaction=nonstopmode -pdf \
        -lualatex -pdflualatex="lualatex -shell-escape %O %S"
    '';

    installPhase = ''
      mkdir -p $out/doc
      cp main.pdf $out/doc/paper.pdf
    '';
  };
