# RE4_EFF_BLOB_RUST

Extract and repack RE4 EFFBLOB files (RE4 2007/UHD/PS2)

**Translate from Portuguese Brazil**

Tool destinada a extrair e recompactar arquivos .EFFBLOB, que são arquivos gerados pela tool "RE4_EFF_SPLIT_TOOL".
<br>Aviso: Tool criada por Zatarita, fork da tool por JaderLink;

## Extract

Use o arquivo .bat para extrair: 
<br> EFFBLOB Extract ALL.bat (todos que estão na pasta)
<br> EFFBLOB Extract One.bat (araste o arquivo .effblob, sobre o .bat)
<br>
<br> Para o exemplo "core_001.EFFBLOB" serão gerados os arquivos:
<br> * "core_001.EFFBLOBTXT" = arquivo usado para o repack;
<br> * "core_001/Tables/Table_0_TPL_Texture_IDs.txt2"
<br> * "core_001/Tables/Table_1_Effect_0_Indexes.txt2"
<br> * "core_001/Tables/Table_2_EAR_Links.txt2"
<br> * "core_001/Tables/Table_3_Effect_Path_IDs.txt2"
<br> * "core_001/Tables/Table_4_BIN_Model_IDs.txt2"
<br> * "core_001/Tables/Table_6_TextureData.txt2"
<br> * "core_001/Tables/Table_9_Paths.txt2"
<br> * "core_001/Effect 0/" = Quantidade de "Effect Group" tem que ser o mesmo definido no arquivo "Table_1_Effect_0_Indexes.txt2";
<br> * "core_001/Effect 1/" = Quantidade de "Effect Group" tem que ser o mesmo definido no arquivo "Table_2_EAR_Links.txt2";
<br> * "core_001/Effect */Effect Group * Data.txt2" = arquivo com os "EffectEntry";
<br> * "core_001/Effect */Effect Group * Data.obj" = arquivo apenas para referência, não usado para o repack (aviso: escala 1/100, sendo Y a altura);

## Repack

Use o arquivo .bat para recompactar:
<br> EFFBLOB Repack ALL.bat (todos que estão na pasta)
<br> EFFBLOB Repack One.bat (araste o arquivo .effblobtxt, sobre o .bat)
<br>
<br> Tendo como exemplo o arquivo anterior, para o repack serão usados os arquivos informados acima.
<br> Nota importante sobre a edição dos arquivos txt2:
<br> * Os campos em decimal devem permanecer em decimal e os em hexadecimal devem permanecer em hexadecimal.
<br> * Os campos nos arquivos serão reconhecidos pela ordem no arquivo e não pelo nome, então não mude os campos de ordem, e também não exclua os campos.
<br> * Não é permitido comentários nos arquivos, pois pode dar conflito no repack.

## For developers

Para compliar o programa, foi usada a seguinte versão do Rust:
<br>Aviso: o código pode não compilar em versões mais recentes.

rustc --version
<br>rustc 1.77.2 (25ef9e3d8 2024-04-09)
<br>cargo --version
<br>cargo 1.77.2 (e52e36006 2024-03-26)

-----

**Tool By Zatarita**
<br>**Fork By JADERLINK**
<br>2024-08-05