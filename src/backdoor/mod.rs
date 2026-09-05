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

// backdoor/mod.rs — backdoor module entry.
//
// Exists so `mod backdoor;` in main.rs resolves. Only HVNC is exported.
// HVNC is a localhost RFB skeleton, not a hidden-desktop VNC.

pub mod hvnc;
