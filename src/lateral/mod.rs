/*
** *********************************************************************
** *********************************************************************
**
**     mm     mmmmmm    mmmmmm       mm     mmm  mmm     mm       mmmm
**    ####    ##""""##  ##""""##    ####     ##mm##     ####    m#""""#
**    ####    ##    ##  ##    ##    ####      ####      ####    ##m
**   ##  ##   #######   #######    ##  ##      ##      ##  ##    "####m
**   ######   ##    ##  ##  "##m   ######     ####     ######        "##
**  m##  ##m  ##mmmm##  ##    ##  m##  ##m   ##  ##   m##  ##m  #mmmmm#"
**  ""    ""  """""""   ""    """ ""    ""  """  """  ""    ""   """""
**
**                 mm           mm     mmmmmm      mmmm
**                 ##          ####    ##""""##  m#""""#
**                 ##          ####    ##    ##  ##m
**                 ##         ##  ##   #######    "####m
**                 ##         ######   ##    ##       "##
**                 ##mmmmmm  m##  ##m  ##mmmm##  #mmmmm#"
**                 """"""""  ""    ""  """""""    """""
**
**                     analyze · reverse · disclose
**
** @abraxas_null
** https://abraxaslabs.tech
** https://github.com/abraxas/veneficus
**
**                           Credit: @YoSoth0
**
**                              DISCLAIMER
**       For educational purposes only. 2026 All rights reversed.
**
** *********************************************************************
** *********************************************************************
*/

// lateral/mod.rs — lateral-movement module glue.
//
// Declared from main.rs (`mod lateral`) but nothing in main constructs
// NtlmCoercer. Entire subtree is compiled and unused.

pub mod ntlm;
